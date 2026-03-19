//! cnb release download 子命令 - 下载 Release 附件

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_bytes;
use cnb_tui::success;
use std::path::Path;

/// 下载 Release 附件
#[derive(Debug, Parser)]
pub struct DownloadArgs {
    /// Tag 名称
    #[arg(value_name = "TAG")]
    pub tag: String,

    /// 文件名
    #[arg(value_name = "FILENAME")]
    pub filename: String,

    /// 保存路径
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// 生成分享下载链接（不下载文件）
    #[arg(long)]
    pub share: bool,
}

/// 执行 release download 命令
pub async fn run(ctx: &AppContext, args: &DownloadArgs) -> Result<()> {
    let client = ctx.api_client()?;

    if args.share {
        // 仅获取分享链接
        let url = client
            .get_release_download_url(&args.tag, &args.filename, true)
            .await?;
        println!("{url}");
        return Ok(());
    }

    // 获取下载 URL
    let url = client
        .get_release_download_url(&args.tag, &args.filename, false)
        .await?;

    // 确定保存路径
    let save_path = if let Some(ref output) = args.output {
        let p = Path::new(output);
        if p.is_dir() {
            p.join(&args.filename)
        } else {
            p.to_path_buf()
        }
    } else {
        Path::new(&args.filename).to_path_buf()
    };

    // 下载文件
    let resp = client.http_client().get(&url).send().await?;
    let status = resp.status().as_u16();
    if !(200..300).contains(&status) {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("下载附件失败 (HTTP {status}): {body}");
    }
    let bytes = resp.bytes().await?;
    std::fs::write(&save_path, &bytes)?;

    success!(
        "已下载 {} ({})",
        save_path.display(),
        format_bytes(i64::try_from(bytes.len()).unwrap_or(i64::MAX))
    );

    Ok(())
}
