//! cnb member group-list 子命令 - 列出组织直接成员

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListGroupMembersOptions;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

use super::repo_list::format_access_level;

/// 列出组织直接成员
#[derive(Debug, Parser)]
pub struct GroupListArgs {
    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 按角色过滤
    #[arg(short = 'r', long = "role")]
    pub role: Option<String>,

    /// 搜索成员
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,
}

/// 执行 member group-list 命令
pub async fn run(ctx: &AppContext, args: &GroupListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let opts = ListGroupMembersOptions {
        page: 1,
        page_size: 100,
        role: args.role.clone(),
        search: args.search.clone(),
    };
    let members = client.list_group_members(&args.group, &opts).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&members)?);
        return Ok(());
    }

    if members.is_empty() {
        eprintln!("没有找到成员");
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
