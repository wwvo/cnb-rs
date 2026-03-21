//! cnb member collaborator-list 子命令 - 列出外部贡献者

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table, info};

use super::repo_list::format_access_level;

/// 列出外部贡献者
#[derive(Debug, Parser)]
pub struct CollaboratorListArgs {
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

/// 执行 member collaborator-list 命令
pub async fn run(ctx: &AppContext, args: &CollaboratorListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let collaborators = client
        .list_outside_collaborators(
            &args.group,
            args.role.as_deref(),
            args.search.as_deref(),
            1,
            100,
        )
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&collaborators)?);
        return Ok(());
    }

    if collaborators.is_empty() {
        info!("没有找到外部贡献者");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("USERNAME", 18),
        Column::new("NICKNAME", 16),
        Column::new("ROLE", 12),
        Column::new("EMAIL", 26),
        Column::new("JOINED", 12),
    ]);

    for c in &collaborators {
        let joined = c.join_time.get(..10).unwrap_or(&c.join_time).to_string();
        table.add_row(vec![
            c.username.clone(),
            c.nickname.clone(),
            format_access_level(&c.access_level),
            c.email.clone(),
            joined,
        ]);
    }

    table.print();

    Ok(())
}
