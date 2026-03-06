//! cnb repo unarchive 子命令 — 解除仓库归档

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 解除仓库归档（恢复可写状态）
#[derive(Debug, Parser)]
pub struct UnarchiveArgs {
    /// 仓库路径（如 org/repo）
    pub repo: String,
}

pub async fn run(ctx: &AppContext, args: &UnarchiveArgs) -> Result<()> {
    let client = ctx.api_client()?;
    client.unarchive_repo(&args.repo).await?;

    success!("仓库已解除归档 ({})", args.repo);

    Ok(())
}
