//! cnb build download-log 子命令 - 下载 Runner 日志

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 下载 Runner 日志
#[derive(Debug, Parser)]
pub struct DownloadLogArgs {
    /// 流水线 ID
    pub pipeline_id: String,

    /// 输出文件路径（不指定则输出到 stdout）
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,
}

/// 执行 build download-log 命令
pub async fn run(ctx: &AppContext, args: &DownloadLogArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let content = client.download_build_log(&args.pipeline_id).await?;

    if let Some(ref path) = args.output {
        std::fs::write(path, &content)?;
        success!("日志已保存到 {path}");
    } else {
        print!("{content}");
    }

    Ok(())
}
