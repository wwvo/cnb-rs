//! cnb registry list 子命令 - 列出组织下的制品库

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListRegistriesOptions;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_bytes;
use cnb_tui::{Column, Table};

/// 列出组织下的制品库
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 制品库类型过滤
    #[arg(short = 't', long = "type")]
    pub registry_type: Option<String>,

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

/// 执行 registry list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let opts = ListRegistriesOptions {
        registry_type: args.registry_type.clone(),
        search: args.search.clone(),
        order_by: args.order_by.clone(),
        desc: args.desc,
        page: 1,
        page_size: 100,
    };
    let registries = client.list_registries(&args.group, &opts).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&registries)?);
        return Ok(());
    }

    if registries.is_empty() {
        eprintln!("没有找到制品库");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NAME", 20),
        Column::new("TYPE", 10),
        Column::new("PACKAGES", 8),
        Column::new("SIZE", 10),
        Column::new("VISIBILITY", 10),
        Column::new("LAST PUSH", 12),
    ]);

    for r in &registries {
        let visibility = if let Some(s) = r.visibility_level.as_str() {
            s.to_string()
        } else {
            r.visibility_level.to_string()
        };
        let last_push = r
            .last_push_time
            .get(..10)
            .unwrap_or(&r.last_push_time)
            .to_string();
        table.add_row(vec![
            r.name.clone(),
            r.kind.clone(),
            r.pkg_count.to_string(),
            format_bytes(i64::try_from(r.used_size).unwrap_or(i64::MAX)),
            visibility,
            last_push,
        ]);
    }

    table.print();

    Ok(())
}
