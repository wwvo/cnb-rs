//! download 命令执行逻辑 - 基础顺序下载（blob 文件）

use anyhow::Result;
use base64::Engine;
use cnb_core::context::AppContext;
use std::path::Path;

use super::DownloadArgs;

/// 待下载文件信息
pub struct DownFile {
    pub path: String,
    /// blob / lfs
    pub file_type: String,
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
        println!("请指定要下载的文件，使用 --files 参数，详见 -h");
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
    let mut download_files: Vec<DownFile> = Vec::new();

    for file_path in &args.files {
        let content = client.get_content(file_path, &git_ref).await?;

        match content.content_type.as_str() {
            "blob" | "lfs" => {
                download_files.push(DownFile {
                    path: content.path,
                    file_type: content.content_type,
                    content: content.content,
                    lfs_download_url: content.lfs_download_url,
                    size: content.size,
                });
            }
            "tree" => {
                // 递归获取目录下的文件
                for entry in &content.entries {
                    if entry.entry_type == "blob" || entry.entry_type == "lfs" {
                        let sub = client.get_content(&entry.path, &git_ref).await?;
                        download_files.push(DownFile {
                            path: sub.path,
                            file_type: sub.content_type,
                            content: sub.content,
                            lfs_download_url: sub.lfs_download_url,
                            size: sub.size,
                        });
                    }
                }
            }
            _ => {
                println!("跳过未知类型: {} ({})", file_path, content.content_type);
            }
        }
    }

    // 去重
    let mut seen = std::collections::HashSet::new();
    download_files.retain(|f| seen.insert(f.path.clone()));

    if download_files.is_empty() {
        println!("没有找到需要下载的文件");
        return Ok(());
    }

    println!("共 {} 个文件待下载", download_files.len());

    // 顺序下载
    let mut success = 0usize;
    let mut failed = 0usize;

    for file in &download_files {
        match file.file_type.as_str() {
            "blob" => match download_blob(file, &args.local_dir) {
                Ok(()) => {
                    println!("✓ {}", file.path);
                    success += 1;
                }
                Err(e) => {
                    println!("✗ {} - {e}", file.path);
                    failed += 1;
                }
            },
            "lfs" => {
                // LFS 支持将在后续提交中实现
                println!("⊘ {} (LFS 文件，暂不支持)", file.path);
            }
            _ => {}
        }
    }

    println!("\n下载完成：{success} 成功，{failed} 失败");

    Ok(())
}

/// 下载 blob 类型文件（base64 解码写入磁盘）
fn download_blob(file: &DownFile, local_dir: &str) -> Result<()> {
    let file_path = Path::new(local_dir).join(&file.path);

    // 创建父目录
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // base64 解码
    let decoded = base64::engine::general_purpose::STANDARD.decode(&file.content)?;

    // 写入文件
    std::fs::write(&file_path, decoded)?;

    Ok(())
}
