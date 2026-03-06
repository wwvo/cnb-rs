//! cnb repo fork 子命令 — 查看仓库的 Fork 列表

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看仓库的 Fork 列表
#[derive(Debug, Parser)]
pub struct ForkArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前 git 目录对应的仓库
    pub repo: Option<String>,

    /// 最大列出数量
    #[arg(short = 'L', long, default_value_t = 30)]
    pub limit: u32,
}

pub async fn run(ctx: &AppContext, args: &ForkArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let fork_list = client.list_forks(repo_path, 1, args.limit.min(100)).await?;

    if fork_list.forks.is_empty() {
        info!("没有找到 Fork");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&fork_list)?);
        return Ok(());
    }

    info!("共 {} 个 Fork", fork_list.fork_tree_count);

    let mut table = Table::new(vec![
        Column::new("仓库路径", 35),
        Column::new("用户", 15),
        Column::new("Fork 数", 8),
        Column::new("创建时间", 12),
    ]);

    for fork in &fork_list.forks {
        let created_date = if fork.created_at.len() >= 10 {
            fork.created_at[..10].to_string()
        } else {
            fork.created_at.clone()
        };

        table.add_row(vec![
            fork.path.clone(),
            fork.username.clone(),
            fork.fork_count.to_string(),
            created_date,
        ]);
    }

    table.print();

    Ok(())
}
