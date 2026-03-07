//! cnb registry package list 子命令 - 列出制品

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

/// 列出制品
#[derive(Debug, Parser)]
pub struct PackageListArgs {
    /// 制品类型（docker/npm/pypi/maven/helm 等）
    pub pkg_type: String,

    /// 制品库路径（组织/制品库）
    #[arg(short = 'r', long = "registry")]
    pub registry: String,

    /// 按名称搜索
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,

    /// 排序方式
    #[arg(long = "order")]
    pub ordering: Option<String>,
}

/// 执行 registry package list 命令
pub async fn run(ctx: &AppContext, args: &PackageListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let packages = client.list_packages(
        &args.registry, &args.pkg_type, args.name.as_deref(),
        args.ordering.as_deref(), 1, 100,
    ).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&packages)?);
        return Ok(());
    }

    if packages.is_empty() {
        eprintln!("没有找到制品");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NAME", 25),
        Column::new("TAGS", 6),
        Column::new("PULLS", 8),
        Column::new("RECENT", 8),
        Column::new("LAST PUSH", 20),
    ]);

    for p in &packages {
        let last_push = if let Some(pusher) = p.last_pusher.as_object() {
            let name = pusher.get("name").and_then(|v| v.as_str()).unwrap_or("-");
            let push_at = pusher.get("push_at").and_then(|v| v.as_str()).unwrap_or("-");
            let date = push_at.get(..10).unwrap_or(push_at);
            format!("{name} ({date})")
        } else {
            "-".to_string()
        };
        table.add_row(vec![
            p.name.clone(),
            p.count.to_string(),
            p.pull_count.to_string(),
            p.recent_pull_count.to_string(),
            last_push,
        ]);
    }

    table.print();

    Ok(())
}
