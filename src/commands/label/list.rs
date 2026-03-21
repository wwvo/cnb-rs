//! cnb label list 子命令 - 列出仓库所有标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

/// 列出仓库所有标签
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 搜索关键字
    #[arg(short = 'k', long = "keyword")]
    pub keyword: Option<String>,
}

/// 执行 label list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let labels = client.list_all_labels(args.keyword.as_deref()).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&labels)?);
        return Ok(());
    }

    if labels.is_empty() {
        info!("没有找到标签");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NAME", 20),
        Column::new("COLOR", 10),
        Column::new("DESCRIPTION", 40),
    ]);

    for label in &labels {
        table.add_row(vec![
            label.name.clone(),
            label.color.clone(),
            label.description.clone(),
        ]);
    }

    table.print();

    Ok(())
}
