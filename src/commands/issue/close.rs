//! cnb issue close 子命令 - 关闭 Issue

use anyhow::Result;
use clap::{Parser, ValueEnum};
use cnb_api::types::UpdateIssueRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// Issue 关闭原因
#[derive(Debug, Clone, ValueEnum)]
pub enum CloseReason {
    /// 已完成
    Completed,
    /// 不计划处理
    NotPlanned,
}

impl std::fmt::Display for CloseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloseReason::Completed => write!(f, "completed"),
            CloseReason::NotPlanned => write!(f, "not_planned"),
        }
    }
}

/// 关闭 Issue
#[derive(Debug, Parser)]
pub struct CloseArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 关闭原因
    #[arg(short = 'r', long = "reason", value_enum, default_value = "completed")]
    pub reason: CloseReason,
}

/// 执行 issue close 命令
pub async fn run(ctx: &AppContext, args: &CloseArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = UpdateIssueRequest {
        state: Some("closed".to_string()),
        state_reason: Some(args.reason.to_string()),
        ..Default::default()
    };

    client.update_issue(&args.number, &req).await?;
    success!("Issue #{} 已关闭（原因: {}）", args.number, args.reason);

    Ok(())
}
