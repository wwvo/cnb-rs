//! cnb issue create 子命令 - 创建 Issue

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateIssueRequest;
use cnb_core::context::AppContext;

/// 创建 Issue
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// Issue 标题
    #[arg(short = 't', long = "title")]
    pub title: String,

    /// Issue 描述
    #[arg(short = 'b', long = "body", default_value = "")]
    pub body: String,

    /// 优先级
    #[arg(short = 'p', long = "priority", default_value = "")]
    pub priority: String,

    /// 标签（逗号分隔）
    #[arg(short = 'l', long = "labels", value_delimiter = ',')]
    pub labels: Vec<String>,

    /// 处理人（逗号分隔）
    #[arg(short = 'a', long = "assignees", value_delimiter = ',')]
    pub assignees: Vec<String>,
}

/// 执行 issue create 命令
pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = CreateIssueRequest {
        title: args.title.clone(),
        body: args.body.clone(),
        priority: args.priority.clone(),
        labels: args.labels.clone(),
        assignees: args.assignees.clone(),
    };

    let issue = client.create_issue(&req).await?;
    println!(
        "{}{}/-/issues/{}",
        client.base_web_url(),
        client.repo(),
        issue.number
    );

    Ok(())
}
