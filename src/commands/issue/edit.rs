//! cnb issue edit 子命令 - 编辑 Issue

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateIssueRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 编辑 Issue
#[derive(Debug, Parser)]
pub struct EditArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 修改标题
    #[arg(short = 't', long)]
    pub title: Option<String>,

    /// 修改描述
    #[arg(short = 'b', long)]
    pub body: Option<String>,

    /// 修改优先级（-2P/-1P/P0/P1/P2/P3）
    #[arg(short = 'p', long)]
    pub priority: Option<String>,

    /// 修改开始日期（YYYY-MM-DD）
    #[arg(long)]
    pub start_date: Option<String>,

    /// 修改结束日期（YYYY-MM-DD）
    #[arg(long)]
    pub end_date: Option<String>,
}

/// 执行 issue edit 命令
pub async fn run(ctx: &AppContext, args: &EditArgs) -> Result<()> {
    // 至少需要指定一个修改项
    if args.title.is_none()
        && args.body.is_none()
        && args.priority.is_none()
        && args.start_date.is_none()
        && args.end_date.is_none()
    {
        anyhow::bail!("请至少指定一个修改项（--title、--body、--priority、--start-date、--end-date）");
    }

    let client = ctx.api_client()?;

    let req = UpdateIssueRequest {
        title: args.title.clone(),
        body: args.body.clone(),
        priority: args.priority.clone(),
        start_date: args.start_date.clone(),
        end_date: args.end_date.clone(),
        ..Default::default()
    };

    client.update_issue(&args.number, &req).await?;
    success!("Issue #{} 已更新", args.number);

    Ok(())
}
