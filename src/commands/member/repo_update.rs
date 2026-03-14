//! cnb member repo-update 子命令 - 更新仓库成员权限

use anyhow::Result;
use clap::Parser;
use cnb_api::types::MemberRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 更新仓库成员权限
#[derive(Debug, Parser)]
pub struct RepoUpdateArgs {
    /// 用户名
    pub username: String,

    /// 新权限等级（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'r', long = "role")]
    pub role: String,
}

/// 执行 member repo-update 命令
pub async fn run(ctx: &AppContext, args: &RepoUpdateArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;

    let req = MemberRequest {
        access_level: args.role.clone(),
        is_outside_collaborator: None,
    };

    client
        .update_repo_member(repo, &args.username, &req)
        .await?;
    success!("已更新 {} 的权限为 {}", args.username, args.role);

    Ok(())
}
