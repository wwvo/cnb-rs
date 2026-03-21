//! info 子命令 - 显示仓库和用户信息

use anyhow::Result;
use clap::Args;
use cnb_core::context::AppContext;

/// 显示当前用户与仓库信息
#[derive(Debug, Args)]
pub struct InfoArgs;

impl InfoArgs {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        run(ctx).await
    }
}

async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;

    // 并行获取用户和仓库信息
    let (user, repo) = tokio::join!(client.me(), client.get_repo());
    let user = user?;
    let repo = repo?;

    if ctx.json() {
        let json = serde_json::json!({
            "user": user,
            "repo": repo,
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
        return Ok(());
    }

    println!("  {:<16} {}", "MyID", user.id);
    println!("  {:<16} {}", "MyUserName", user.username);
    println!("  {:<16} {}", "MyNickName", user.nickname);
    println!("  {:<16} {}", "MyEmail", user.email);
    println!("  {:<16} {}", "RepoName", repo.path);
    println!("  {:<16} {}", "RepoID", repo.id);
    println!("  {:<16} {}", "RepoLicense", repo.license);
    println!("  {:<16} {}", "RepoStars", repo.star_count);
    println!("  {:<16} {}", "RepoForks", repo.fork_count);
    println!("  {:<16} {}", "RepoDescription", repo.description);

    Ok(())
}
