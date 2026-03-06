//! cnb pull list 子命令 - 列出与我相关的 Pull Request

use anyhow::Result;
use clap::{Parser, ValueEnum};
use cnb_api::types::ListPullsOptions;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

/// 列表状态过滤
#[derive(Debug, Clone, ValueEnum)]
pub enum StateFilter {
    Open,
    Closed,
    All,
}

impl std::fmt::Display for StateFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateFilter::Open => write!(f, "open"),
            StateFilter::Closed => write!(f, "closed"),
            StateFilter::All => write!(f, "all"),
        }
    }
}

/// 列出 Pull Request
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 按状态过滤
    #[arg(short = 's', long = "state", value_enum, default_value = "open")]
    pub state: StateFilter,

    /// 按作者过滤（默认为当前用户）
    #[arg(long = "author")]
    pub author: Option<String>,

    /// 按评审人过滤（默认为当前用户）
    #[arg(long = "reviewer")]
    pub reviewer: Option<String>,
}

/// 执行 pull list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let state = args.state.to_string();

    // 指定了 author 或 reviewer 时，直接按指定条件查询
    if args.author.is_some() || args.reviewer.is_some() {
        let opts = ListPullsOptions {
            state,
            page: 1,
            page_size: 100,
            authors: args.author.clone(),
            reviewers: args.reviewer.clone(),
        };
        let pulls = client.list_pulls(&opts).await?;

        if pulls.is_empty() {
            info!("没有找到符合条件的 Pull Request");
            return Ok(());
        }

        if ctx.json() {
            println!("{}", serde_json::to_string_pretty(&pulls)?);
            return Ok(());
        }

        let mut table = Table::new(vec![
            Column::new("NUMBER", 15),
            Column::new("TITLE", 55),
            Column::new("STATE", 10),
            Column::new("BLOCKEDON", 15),
        ]);
        for pr in &pulls {
            table.add_row(vec![
                format!("#{}", pr.number),
                pr.title.clone(),
                pr.state.clone(),
                pr.blocked_on.clone(),
            ]);
        }
        table.print();
        return Ok(());
    }

    // 默认行为：查询与当前用户相关的 PR
    let me = client.me().await?;

    let from_me_opts = ListPullsOptions {
        state: state.clone(),
        page: 1,
        page_size: 100,
        authors: Some(me.username.clone()),
        ..Default::default()
    };
    let to_me_opts = ListPullsOptions {
        state,
        page: 1,
        page_size: 100,
        reviewers: Some(me.username.clone()),
        ..Default::default()
    };

    let (from_me, to_me) = tokio::join!(
        client.list_pulls(&from_me_opts),
        client.list_pulls(&to_me_opts)
    );
    let from_me = from_me?;
    let to_me = to_me?;

    // 合并并去重：同一 PR 既是我创建的又需要我评审时标记为 ME->ME
    let mut results: Vec<(String, String, String, &str)> = Vec::new();
    for pr in &from_me {
        results.push((pr.number.clone(), pr.title.clone(), pr.blocked_on.clone(), "ME->"));
    }
    for pr in &to_me {
        if let Some(existing) = results.iter_mut().find(|(n, _, _, _)| *n == pr.number) {
            existing.3 = "ME->ME";
        } else {
            results.push((pr.number.clone(), pr.title.clone(), pr.blocked_on.clone(), "->ME"));
        }
    }

    if results.is_empty() {
        info!("没有找到与我相关的 Pull Request");
        return Ok(());
    }

    if ctx.json() {
        let mut all_pulls = from_me;
        all_pulls.extend(to_me);
        // 去重
        let mut seen = std::collections::HashSet::new();
        all_pulls.retain(|p| seen.insert(p.number.clone()));
        println!("{}", serde_json::to_string_pretty(&all_pulls)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NUMBER", 15),
        Column::new("TITLE", 55),
        Column::new("BLOCKEDON", 15),
        Column::new("TYPE", 10),
    ]);
    for (number, title, blocked_on, pr_type) in &results {
        table.add_row(vec![
            format!("#{number}"),
            title.to_string(),
            blocked_on.to_string(),
            pr_type.to_string(),
        ]);
    }
    table.print();

    Ok(())
}
