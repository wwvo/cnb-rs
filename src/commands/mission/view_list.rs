//! cnb mission view list 子命令 - 列出视图

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

/// 列出任务集视图
#[derive(Debug, Parser)]
pub struct ViewListArgs {
    /// 任务集路径
    pub mission: String,
}

/// 执行 mission view list 命令
pub async fn run(ctx: &AppContext, args: &ViewListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let views = client.list_mission_views(&args.mission).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&views)?);
        return Ok(());
    }

    if views.is_empty() {
        info!("没有找到视图");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("ID", 16),
        Column::new("NAME", 20),
        Column::new("TYPE", 12),
    ]);

    for v in &views {
        let view_type = if let Some(s) = v.view_type.as_str() {
            s.to_string()
        } else {
            v.view_type.to_string()
        };
        table.add_row(vec![v.id.clone(), v.name.clone(), view_type]);
    }

    table.print();

    Ok(())
}
