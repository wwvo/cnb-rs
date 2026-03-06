//! cnb group sub-groups 子命令 - 列出子组织

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListSubGroupsOptions;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 列出子组织
#[derive(Debug, Parser)]
pub struct SubGroupsArgs {
    /// 父组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 关键字过滤
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,
}

pub async fn run(ctx: &AppContext, args: &SubGroupsArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let opts = ListSubGroupsOptions {
        page: 1,
        page_size: 100,
        search: args.search.clone(),
    };

    let groups = client.list_subgroups(&args.group, &opts).await?;

    if groups.is_empty() {
        info!("没有找到子组织");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&groups)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("路径", 30),
        Column::new("描述", 30),
        Column::new("仓库", 6),
        Column::new("成员", 6),
        Column::new("子组织", 6),
    ]);

    for g in &groups {
        table.add_row(vec![
            g.path.clone(),
            g.description.clone(),
            g.sub_repo_count.to_string(),
            g.member_count.to_string(),
            g.sub_group_count.to_string(),
        ]);
    }

    table.print();

    Ok(())
}
