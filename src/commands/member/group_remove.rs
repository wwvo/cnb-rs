//! cnb member group-remove 子命令 - 移除组织成员

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 移除组织成员
#[derive(Debug, Parser)]
pub struct GroupRemoveArgs {
    /// 用户名
    pub username: String,

    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 member group-remove 命令
pub async fn run(ctx: &AppContext, args: &GroupRemoveArgs) -> Result<()> {
    let client = ctx.api_client()?;

    confirm_action(&format!("确认从组织 {} 移除成员 {}？", args.group, args.username), args.yes)?;

    client.remove_group_member(&args.group, &args.username).await?;
    success!("已从组织 {} 移除成员 {}", args.group, args.username);

    Ok(())
}
