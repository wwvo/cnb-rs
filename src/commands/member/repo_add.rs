//! cnb member repo-add 子命令 - 添加仓库成员

use anyhow::Result;
use clap::Parser;
use cnb_api::types::MemberRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 添加仓库成员
#[derive(Debug, Parser)]
pub struct RepoAddArgs {
    /// 用户名
    pub username: String,

    /// 权限等级（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'r', long = "role")]
    pub role: String,

    /// 标记为外部贡献者
    #[arg(long = "outside-collaborator", default_value_t = false)]
    pub outside_collaborator: bool,
}

/// 执行 member repo-add 命令
pub async fn run(ctx: &AppContext, args: &RepoAddArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;

    let req = MemberRequest {
        access_level: args.role.clone(),
        is_outside_collaborator: if args.outside_collaborator {
            Some(true)
        } else {
            None
        },
    };

    client.add_repo_member(repo, &args.username, &req).await?;

    if args.outside_collaborator {
        success!("已添加 {} 为外部贡献者 ({})", args.username, args.role);
    } else {
        success!("已添加 {} 为仓库成员 ({})", args.username, args.role);
    }

    Ok(())
}
