//! download 命令执行逻辑 - 并发下载 + 进度条

use anyhow::Result;
use base64::Engine;
use cnb_core::context::AppContext;
use cnb_tui::info;
use futures::future::try_join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;

use cnb_api::types::ContentType;

use super::DownloadArgs;

/// 待下载文件信息
pub struct DownFile {
    pub path: String,
    pub file_type: ContentType,
    /// base64 编码内容（blob 类型）
    pub content: String,
    /// LFS 下载 URL（lfs 类型）
    pub lfs_download_url: String,
    /// 文件大小
    pub size: i64,
}

/// 执行 download 命令
pub async fn run(ctx: &AppContext, args: &DownloadArgs) -> Result<()> {
    if args.files.is_empty() {
        info!("请指定要下载的文件，使用 --files 参数，详见 -h");
        return Ok(());
    }

    let client = ctx.api_client()?;

    // 确定 git ref
    let git_ref = if args.git_ref.is_empty() {
        "HEAD".to_string()
    } else {
        args.git_ref.clone()
    };

    // 收集需要下载的文件
    let all_files = collect_files(client, &args.files, &git_ref).await?;

    // 应用 include/exclude glob 过滤
    let download_files = filter_files(all_files, &args.include, &args.exclude);

    if download_files.is_empty() {
        info!("没有找到需要下载的文件");
        return Ok(());
    }

    eprintln!("共 {} 个文件待下载\n", download_files.len());

    // 并发下载 + 进度条
    let mp = MultiProgress::new();
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let local_dir = args.local_dir.clone();
    let token = client.token().to_string();
    let http = client.http_client().clone();

    let mut handles = Vec::new();

    for file in download_files {
        let sem = semaphore.clone();
        let mp = mp.clone();
        let local_dir = local_dir.clone();
        let token = token.clone();
        let http = http.clone();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await;

            let progress_len = u64::try_from(file.size).unwrap_or(0).max(1);
            let pb = mp.add(ProgressBar::new(progress_len));
            pb.set_style(
                ProgressStyle::with_template(
                    "{prefix:.cyan} [{bar:30}] {bytes}/{total_bytes} {msg}",
                )
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("=> "),
            );
            let display_name = truncate_filename(&file.path, 30);
            pb.set_prefix(display_name);

            let result = match file.file_type {
                ContentType::Blob => download_blob(&file, &local_dir, &pb),
                ContentType::Lfs => download_lfs(&http, &file, &local_dir, &token, &pb).await,
                ContentType::Tree => Ok(()),
            };

            match &result {
                Ok(()) => {
                    pb.set_message("done");
                    pb.finish();
                }
                Err(e) => {
                    pb.set_message(format!("FAILED: {e}"));
                    pb.abandon();
                }
            }

            (file.path, result)
        });

        handles.push(handle);
    }

    // 等待所有下载完成
    let mut success = 0usize;
    let mut failed = 0usize;
    let mut errors = Vec::new();

    for handle in handles {
        match handle.await {
            Ok((path, result)) => match result {
                Ok(()) => success += 1,
                Err(e) => {
                    errors.push(format!("{path}: {e}"));
                    failed += 1;
                }
            },
            Err(e) => {
                errors.push(format!("下载任务执行失败: {e}"));
                failed += 1;
            }
        }
    }

    eprintln!("\n下载完成：{success} 成功，{failed} 失败");
    for err in &errors {
        eprintln!("  ✗ {err}");
    }

    if failed > 0 {
        anyhow::bail!("下载过程中有 {failed} 个失败项");
    }

    Ok(())
}

/// 收集需要下载的文件列表（两阶段并行获取）
async fn collect_files(
    client: &cnb_api::client::CnbClient,
    files: &[String],
    git_ref: &str,
) -> Result<Vec<DownFile>> {
    let mut download_files: Vec<DownFile> = Vec::new();
    let mut pending_paths: Vec<String> = files.to_vec();

    while !pending_paths.is_empty() {
        let contents = try_join_all(
            pending_paths
                .iter()
                .map(|path| client.get_content(path, git_ref)),
        )
        .await?;
        pending_paths.clear();

        for content in contents {
            match content.content_type {
                ContentType::Blob | ContentType::Lfs => {
                    download_files.push(DownFile {
                        path: content.path,
                        file_type: content.content_type,
                        content: content.content,
                        lfs_download_url: content.lfs_download_url,
                        size: content.size,
                    });
                }
                ContentType::Tree => {
                    for entry in &content.entries {
                        pending_paths.push(entry.path.clone());
                    }
                }
            }
        }
    }

    // 去重
    let mut seen = std::collections::HashSet::new();
    download_files.retain(|f| seen.insert(f.path.clone()));

    Ok(download_files)
}

/// 下载 blob 类型文件（base64 解码写入磁盘）
fn download_blob(file: &DownFile, local_dir: &str, pb: &ProgressBar) -> Result<()> {
    let file_path = Path::new(local_dir).join(&file.path);

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let decoded = base64::engine::general_purpose::STANDARD.decode(&file.content)?;
    std::fs::write(&file_path, &decoded)?;
    pb.set_length(decoded.len() as u64);
    pb.set_position(decoded.len() as u64);

    Ok(())
}

/// 下载 LFS 类型文件（HTTP 流式下载）
async fn download_lfs(
    http: &reqwest::Client,
    file: &DownFile,
    local_dir: &str,
    token: &str,
    pb: &ProgressBar,
) -> Result<()> {
    let file_path = Path::new(local_dir).join(&file.path);

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut resp = http
        .get(&file.lfs_download_url)
        .header("Accept", "application/vnd.cnb.api+json")
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;

    let status = resp.status().as_u16();
    if !(200..300).contains(&status) {
        anyhow::bail!("HTTP {status}");
    }

    if let Some(len) = resp.content_length() {
        pb.set_length(len);
    }

    let mut out = std::fs::File::create(&file_path)?;
    while let Some(chunk) = resp.chunk().await? {
        out.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }

    Ok(())
}

/// 根据 include/exclude glob 模式过滤文件列表
fn filter_files(files: Vec<DownFile>, include: &[String], exclude: &[String]) -> Vec<DownFile> {
    files
        .into_iter()
        .filter(|file| {
            // include：空列表表示包含所有
            let included = if include.is_empty() {
                true
            } else {
                include.iter().any(|pattern| {
                    glob::Pattern::new(pattern)
                        .map(|p| p.matches(&file.path))
                        .unwrap_or(false)
                })
            };

            // exclude：匹配任一排除模式则排除
            let excluded = exclude.iter().any(|pattern| {
                glob::Pattern::new(pattern)
                    .map(|p| p.matches(&file.path))
                    .unwrap_or(false)
            });

            included && !excluded
        })
        .collect()
}

/// 截断文件名显示（UTF-8 安全）
fn truncate_filename(path: &str, max_len: usize) -> String {
    let char_count = path.chars().count();
    if char_count <= max_len {
        return path.to_string();
    }
    let half = (max_len - 3) / 2;
    let prefix: String = path.chars().take(half).collect();
    let suffix: String = path.chars().skip(char_count - half).collect();
    format!("{prefix}...{suffix}")
}
