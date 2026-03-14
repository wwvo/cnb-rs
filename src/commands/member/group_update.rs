//! cnb member group-update 子命令 - 更新组织成员权限

use anyhow::Result;
use clap::Parser;
use cnb_api::types::GroupMemberRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 更新组织成员权限
#[derive(Debug, Parser)]
pub struct GroupUpdateArgs {
    /// 用户名
    pub username: String,

    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 新权限等级（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'r', long = "role")]
    pub role: String,
}

/// 执行 member group-update 命令
pub async fn run(ctx: &AppContext, args: &GroupUpdateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = GroupMemberRequest {
        access_level: args.role.clone(),
        is_outside_collaborator: false,
    };

    client
        .update_group_member(&args.group, &args.username, &req)
        .await?;
    success!("已更新 {} 的权限为 {}", args.username, args.role);

    Ok(())
}
