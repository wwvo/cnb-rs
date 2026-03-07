//! cnb label issue-add 子命令 - 为 Issue 添加标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::IssueLabelRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 为 Issue 添加标签
#[derive(Debug, Parser)]
pub struct IssueAddArgs {
    /// Issue 编号
    pub number: i64,

    /// 标签名称（可多次指定）
    #[arg(short = 'l', long = "labels", value_delimiter = ',')]
    pub labels: Vec<String>,
}

/// 执行 label issue-add 命令
pub async fn run(ctx: &AppContext, args: &IssueAddArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = IssueLabelRequest {
        labels: args.labels.clone(),
    };

    client.add_issue_labels(&args.number.to_string(), &req).await?;

    success!("已为 Issue #{} 添加标签: {}", args.number, args.labels.join(", "));

    Ok(())
}
