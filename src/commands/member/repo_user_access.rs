//! cnb member repo-user-access 子命令 - 查看指定成员在仓库的权限层级

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

use super::repo_list::format_access_level;

/// 查看指定成员在仓库的权限层级
#[derive(Debug, Parser)]
pub struct RepoUserAccessArgs {
    /// 用户名
    pub username: String,
}

/// 执行 member repo-user-access 命令
pub async fn run(ctx: &AppContext, args: &RepoUserAccessArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;
    let levels = client.get_repo_user_access(repo, &args.username).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&levels)?);
        return Ok(());
    }

    if levels.is_empty() {
        info!("没有找到权限信息");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("PATH", 30),
        Column::new("ACCESS LEVEL", 15),
    ]);

    for level in &levels {
        table.add_row(vec![
            level.path.clone(),
            format_access_level(&level.access_level),
        ]);
    }

    table.print();

    Ok(())
}
