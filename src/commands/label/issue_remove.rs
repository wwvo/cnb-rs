//! cnb label issue-remove 子命令 - 移除 Issue 单个标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 移除 Issue 单个标签
#[derive(Debug, Parser)]
pub struct IssueRemoveArgs {
    /// Issue 编号
    pub number: i64,

    /// 标签名称
    pub name: String,
}

/// 执行 label issue-remove 命令
pub async fn run(ctx: &AppContext, args: &IssueRemoveArgs) -> Result<()> {
    let client = ctx.api_client()?;

    client.remove_issue_label(&args.number.to_string(), &args.name).await?;

    success!("已从 Issue #{} 移除标签: {}", args.number, args.name);

    Ok(())
}
