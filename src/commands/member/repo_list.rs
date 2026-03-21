//! cnb member repo-list 子命令 - 列出仓库直接成员

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table, info};

/// 列出仓库直接成员
#[derive(Debug, Parser)]
pub struct RepoListArgs {
    /// 按角色过滤
    #[arg(short = 'r', long = "role")]
    pub role: Option<String>,

    /// 搜索成员
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,
}

/// 格式化权限等级
pub fn format_access_level(val: &serde_json::Value) -> String {
    if let Some(s) = val.as_str() {
        return s.to_string();
    }
    if let Some(n) = val.as_i64() {
        return match n {
            10 => "Guest",
            20 => "Reporter",
            30 => "Developer",
            40 => "Master",
            50 => "Owner",
            _ => "Unknown",
        }
        .to_string();
    }
    val.to_string()
}

/// 执行 member repo-list 命令
pub async fn run(ctx: &AppContext, args: &RepoListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;
    let members = client
        .list_repo_members(repo, args.role.as_deref(), args.search.as_deref(), 1, 100)
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&members)?);
        return Ok(());
    }

    if members.is_empty() {
        info!("没有找到成员");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("USERNAME", 18),
        Column::new("NICKNAME", 16),
        Column::new("ROLE", 12),
        Column::new("EMAIL", 26),
        Column::new("JOINED", 12),
    ]);

    for m in &members {
        let joined = m.join_time.get(..10).unwrap_or(&m.join_time).to_string();
        table.add_row(vec![
            m.username.clone(),
            m.nickname.clone(),
            format_access_level(&m.access_level),
            m.email.clone(),
            joined,
        ]);
    }

    table.print();

    Ok(())
}
