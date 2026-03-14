//! cnb mission list 子命令 - 列出组织下的任务集

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

/// 列出组织下的任务集
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 搜索关键字
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    /// 排序字段
    #[arg(long = "order-by")]
    pub order_by: Option<String>,

    /// 降序排列
    #[arg(long = "desc", default_value_t = false)]
    pub desc: bool,
}

/// 执行 mission list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let missions = client
        .list_missions(
            &args.group,
            args.search.as_deref(),
            args.order_by.as_deref(),
            args.desc,
            1,
            100,
        )
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&missions)?);
        return Ok(());
    }

    if missions.is_empty() {
        eprintln!("没有找到任务集");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NAME", 20),
        Column::new("PATH", 30),
        Column::new("VISIBILITY", 10),
        Column::new("PINNED", 6),
        Column::new("CREATED", 12),
    ]);

    for m in &missions {
        let visibility = if let Some(s) = m.visibility_level.as_str() {
            s.to_string()
        } else {
            m.visibility_level.to_string()
        };
        let pinned = if m.pinned { "✓" } else { "✗" };
        let created = m.created_at.get(..10).unwrap_or(&m.created_at).to_string();
        table.add_row(vec![
            m.name.clone(),
            m.path.clone(),
            visibility,
            pinned.to_string(),
            created,
        ]);
    }

    table.print();

    Ok(())
}
