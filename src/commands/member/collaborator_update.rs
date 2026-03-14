//! cnb member collaborator-update 子命令 - 更新外部贡献者权限

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 更新外部贡献者权限
#[derive(Debug, Parser)]
pub struct CollaboratorUpdateArgs {
    /// 用户名
    pub username: String,

    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 新权限等级（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'r', long = "role")]
    pub role: String,
}

/// 执行 member collaborator-update 命令
pub async fn run(ctx: &AppContext, args: &CollaboratorUpdateArgs) -> Result<()> {
    let client = ctx.api_client()?;
    client
        .update_outside_collaborator(&args.group, &args.username, &args.role)
        .await?;
    success!("已更新外部贡献者 {} 的权限为 {}", args.username, args.role);

    Ok(())
}
