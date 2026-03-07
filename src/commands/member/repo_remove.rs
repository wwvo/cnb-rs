//! cnb member repo-remove 子命令 - 移除仓库成员

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 移除仓库成员
#[derive(Debug, Parser)]
pub struct RepoRemoveArgs {
    /// 用户名
    pub username: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 member repo-remove 命令
pub async fn run(ctx: &AppContext, args: &RepoRemoveArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;

    confirm_action(&format!("确认移除成员 {}？", args.username), args.yes)?;

    client.remove_repo_member(&repo, &args.username).await?;
    success!("已移除成员 {}", args.username);

    Ok(())
}
