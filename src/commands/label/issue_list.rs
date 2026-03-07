//! cnb label issue-list 子命令 - 列出 Issue 标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 列出 Issue 标签
#[derive(Debug, Parser)]
pub struct IssueListArgs {
    /// Issue 编号
    pub number: i64,
}

/// 执行 label issue-list 命令
pub async fn run(ctx: &AppContext, args: &IssueListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let labels = client.list_issue_labels(&args.number.to_string()).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&labels)?);
        return Ok(());
    }

    if labels.is_empty() {
        eprintln!("Issue #{} 没有标签", args.number);
        return Ok(());
    }

    for label in &labels {
        println!("{}", label.name);
    }

    Ok(())
}
