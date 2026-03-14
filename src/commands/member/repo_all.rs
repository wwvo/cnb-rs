//! cnb member repo-all 子命令 - 列出仓库所有有效成员

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListRepoAllMembersOptions;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

use super::repo_list::format_access_level;

/// 列出仓库所有有效成员（含继承）
#[derive(Debug, Parser)]
pub struct RepoAllArgs {
    /// 按角色过滤
    #[arg(short = 'r', long = "role")]
    pub role: Option<String>,

    /// 搜索成员
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    /// 精准匹配用户名（逗号分隔）
    #[arg(long = "names")]
    pub names: Option<String>,

    /// 排序字段
    #[arg(long = "order-by")]
    pub order_by: Option<String>,

    /// 降序排列
    #[arg(long = "desc", default_value_t = false)]
    pub desc: bool,
}

/// 执行 member repo-all 命令
pub async fn run(ctx: &AppContext, args: &RepoAllArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;
    let opts = ListRepoAllMembersOptions {
        role: args.role.clone(),
        search: args.search.clone(),
        names: args.names.clone(),
        order_by: args.order_by.clone(),
        desc: args.desc,
        page: 1,
        page_size: 100,
    };
    let members = client.list_repo_all_members(repo, &opts).await?;

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
