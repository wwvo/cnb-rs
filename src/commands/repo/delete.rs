//! cnb repo delete 子命令 — 删除仓库

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};

/// 删除仓库
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// 仓库路径（如 org/repo）
    pub repo: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    if !confirm_action(&format!("确定删除 {} ？此操作不可逆", args.repo), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.delete_repo(&args.repo).await?;

    success!("仓库 {} 已删除", args.repo);

    Ok(())
}
