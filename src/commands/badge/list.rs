//! cnb badge list 子命令 - 列出仓库徽章

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table, info};

/// 列出仓库徽章
#[derive(Debug, Parser)]
pub struct ListArgs {}

/// 执行 badge list 命令
pub async fn run(ctx: &AppContext, _args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let result = client.list_badges().await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    if result.badges.is_empty() {
        info!("没有找到徽章");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NAME", 30),
        Column::new("TYPE", 12),
        Column::new("STATUS", 10),
        Column::new("URL", 50),
    ]);

    for badge in &result.badges {
        let status = badge
            .group
            .as_ref()
            .map_or("-".to_string(), |g| g.status.clone());
        table.add_row(vec![
            badge.name.clone(),
            badge.badge_type.clone(),
            status,
            badge.url.clone(),
        ]);
    }

    table.print();

    Ok(())
}
