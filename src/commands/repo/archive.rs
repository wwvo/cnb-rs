//! cnb repo archive 子命令 — 归档仓库

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};

/// 归档仓库（变为只读状态）
#[derive(Debug, Parser)]
pub struct ArchiveArgs {
    /// 仓库路径（如 org/repo）
    pub repo: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

pub async fn run(ctx: &AppContext, args: &ArchiveArgs) -> Result<()> {
    if !confirm_action(
        &format!("确定归档 {} ？归档后仓库将变为只读", args.repo),
        args.yes,
    )? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.archive_repo(&args.repo).await?;

    success!("仓库已归档 ({})", args.repo);

    Ok(())
}
