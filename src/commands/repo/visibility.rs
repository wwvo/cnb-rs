//! cnb repo visibility 子命令 — 修改仓库可见性

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};

/// 修改仓库可见性
#[derive(Debug, Parser)]
pub struct VisibilityArgs {
    /// 仓库路径（如 org/repo）
    pub repo: String,

    /// 目标可见性（public/private/secret）
    pub visibility: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

pub async fn run(ctx: &AppContext, args: &VisibilityArgs) -> Result<()> {
    // 校验 visibility 值
    let valid = ["public", "private", "secret"];
    if !valid.contains(&args.visibility.as_str()) {
        anyhow::bail!(
            "无效的可见性：{}，可选值：{}",
            args.visibility,
            valid.join(" / ")
        );
    }

    if !confirm_action(
        &format!("确定将 {} 的可见性修改为 {} ？", args.repo, args.visibility),
        args.yes,
    )? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client
        .set_repo_visibility(&args.repo, &args.visibility)
        .await?;

    success!("仓库可见性已修改为 {} ({})", args.visibility, args.repo);

    Ok(())
}
