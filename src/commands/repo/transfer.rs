//! cnb repo transfer 子命令 — 转移仓库到另一个组织

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};

/// 转移仓库到另一个组织
#[derive(Debug, Parser)]
pub struct TransferArgs {
    /// 仓库路径（如 org/repo）
    pub repo: String,

    /// 目标组织
    #[arg(short, long)]
    pub target: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

pub async fn run(ctx: &AppContext, args: &TransferArgs) -> Result<()> {
    if !confirm_action(
        &format!("确定将 {} 转移到 {} ？", args.repo, args.target),
        args.yes,
    )? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.transfer_repo(&args.repo, &args.target).await?;

    // 推断新路径
    let repo_name = args.repo.rsplit('/').next().unwrap_or(&args.repo);
    success!("仓库已转移到 {}/{}", args.target, repo_name);

    Ok(())
}
