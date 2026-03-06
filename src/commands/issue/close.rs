//! cnb issue close 子命令 - 关闭 Issue

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateIssueRequest;
use cnb_core::context::AppContext;

/// 关闭 Issue
#[derive(Debug, Parser)]
pub struct CloseArgs {
    /// Issue 编号
    #[arg(short = 'n', long = "number")]
    pub number: String,

    /// 关闭原因（completed 或 not_planned）
    #[arg(short = 'r', long = "reason", default_value = "completed")]
    pub reason: String,
}

/// 执行 issue close 命令
pub async fn run(ctx: &AppContext, args: &CloseArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = UpdateIssueRequest {
        state: Some("closed".to_string()),
        state_reason: Some(args.reason.clone()),
    };

    client.update_issue(&args.number, &req).await?;
    println!("Issue #{} 已关闭（原因: {}）", args.number, args.reason);

    Ok(())
}
