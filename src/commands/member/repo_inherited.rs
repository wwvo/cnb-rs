//! cnb member repo-inherited 子命令 - 列出仓库继承成员

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

use super::repo_list::format_access_level;

/// 列出仓库继承成员
#[derive(Debug, Parser)]
pub struct RepoInheritedArgs {
    /// 按角色过滤
    #[arg(short = 'r', long = "role")]
    pub role: Option<String>,

    /// 搜索成员
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,
}

/// 执行 member repo-inherited 命令
pub async fn run(ctx: &AppContext, args: &RepoInheritedArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;
    let groups = client
        .list_repo_inherited_members(repo, args.role.as_deref(), args.search.as_deref(), 1, 100)
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&groups)?);
        return Ok(());
    }

    if groups.is_empty() {
        eprintln!("没有找到继承成员");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("INHERIT FROM", 24),
        Column::new("USERNAME", 18),
        Column::new("NICKNAME", 16),
        Column::new("ROLE", 12),
    ]);

    for group in &groups {
        for user in &group.users {
            table.add_row(vec![
                group.inherit_path.clone(),
                user.username.clone(),
                user.nickname.clone(),
                format_access_level(&user.access_level),
            ]);
        }
    }

    table.print();

    Ok(())
}
