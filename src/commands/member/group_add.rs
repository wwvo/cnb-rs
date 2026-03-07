//! cnb member group-add 子命令 - 添加组织成员

use anyhow::Result;
use clap::Parser;
use cnb_api::types::GroupMemberRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 添加组织成员
#[derive(Debug, Parser)]
pub struct GroupAddArgs {
    /// 用户名
    pub username: String,

    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 权限等级（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'r', long = "role")]
    pub role: String,
}

/// 执行 member group-add 命令
pub async fn run(ctx: &AppContext, args: &GroupAddArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = GroupMemberRequest {
        access_level: args.role.clone(),
        is_outside_collaborator: false,
    };

    client.add_group_member(&args.group, &args.username, &req).await?;
    success!("已添加 {} 为组织成员 ({})", args.username, args.role);

    Ok(())
}
