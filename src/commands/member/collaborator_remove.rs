//! cnb member collaborator-remove 子命令 - 移除外部贡献者

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 移除外部贡献者
#[derive(Debug, Parser)]
pub struct CollaboratorRemoveArgs {
    /// 用户名
    pub username: String,

    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 member collaborator-remove 命令
pub async fn run(ctx: &AppContext, args: &CollaboratorRemoveArgs) -> Result<()> {
    let client = ctx.api_client()?;

    confirm_action(&format!("确认移除外部贡献者 {}？", args.username), args.yes)?;

    client.remove_outside_collaborator(&args.group, &args.username).await?;
    success!("已移除外部贡献者 {}", args.username);

    Ok(())
}
