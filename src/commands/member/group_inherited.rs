//! cnb member group-inherited 子命令 - 列出组织继承成员

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListGroupMembersOptions;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table, info};

use super::repo_list::format_access_level;

/// 列出组织继承成员
#[derive(Debug, Parser)]
pub struct GroupInheritedArgs {
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

/// 执行 member group-inherited 命令
pub async fn run(ctx: &AppContext, args: &GroupInheritedArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let opts = ListGroupMembersOptions {
        page: 1,
        page_size: 100,
        role: args.role.clone(),
        search: args.search.clone(),
    };
    let groups = client.list_inherit_members(&args.group, &opts).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&groups)?);
        return Ok(());
    }

    if groups.is_empty() {
        info!("没有找到继承成员");
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
