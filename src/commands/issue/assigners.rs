//! cnb issue assigners 子命令 - 获取和添加 Issue 处理人

use anyhow::Result;
use clap::Parser;
use cnb_api::types::AddAssigneesRequest;
use cnb_core::context::AppContext;
use cnb_tui::{info, success};

/// Issue 处理人管理
#[derive(Debug, Parser)]
pub struct AssignersArgs {
    #[command(subcommand)]
    pub action: AssignersAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum AssignersAction {
    /// 获取 Issue 的处理人
    Get(GetAssignersArgs),

    /// 添加 Issue 处理人
    Add(AddAssignersArgs),
}

/// 获取处理人参数
#[derive(Debug, Parser)]
pub struct GetAssignersArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,
}

/// 添加处理人参数
#[derive(Debug, Parser)]
pub struct AddAssignersArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 处理人用户名（逗号分隔）
    #[arg(short = 'a', long = "assignees", value_delimiter = ',')]
    pub assignees: Vec<String>,
}

/// 执行 assigners 命令
pub async fn run(ctx: &AppContext, args: &AssignersArgs) -> Result<()> {
    match &args.action {
        AssignersAction::Get(get_args) => run_get(ctx, get_args).await,
        AssignersAction::Add(add_args) => run_add(ctx, add_args).await,
    }
}

/// 获取 Issue 处理人
async fn run_get(ctx: &AppContext, args: &GetAssignersArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let assignees = client.list_issue_assignees(&args.number).await?;

    if assignees.is_empty() {
        info!("Issue #{} 没有处理人", args.number);
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&assignees)?);
        return Ok(());
    }

    for assignee in &assignees {
        println!("{}", assignee.username);
    }

    Ok(())
}

/// 添加 Issue 处理人
async fn run_add(ctx: &AppContext, args: &AddAssignersArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let assignee_list: Vec<String> = args
        .assignees
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    if assignee_list.is_empty() {
        anyhow::bail!("没有有效的处理人");
    }

    let req = AddAssigneesRequest {
        assignees: assignee_list,
    };

    client.add_issue_assignees(&args.number, &req).await?;
    success!("Issue #{} 处理人已更新", args.number);

    Ok(())
}
