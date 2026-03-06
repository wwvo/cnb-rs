//! cnb pull update 子命令 - 更新 Pull Request

use anyhow::{Result, bail};
use clap::{Parser, ValueEnum};
use cnb_api::types::UpdatePullRequest;
use cnb_core::context::AppContext;

/// PR 可修改的状态
#[derive(Debug, Clone, ValueEnum)]
pub enum PullUpdateState {
    Open,
    Closed,
}

impl std::fmt::Display for PullUpdateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullUpdateState::Open => write!(f, "open"),
            PullUpdateState::Closed => write!(f, "closed"),
        }
    }
}

/// 更新 Pull Request
#[derive(Debug, Parser)]
pub struct UpdateArgs {
    /// PR 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 修改标题
    #[arg(short = 't', long = "title")]
    pub title: Option<String>,

    /// 修改描述
    #[arg(short = 'b', long = "body")]
    pub body: Option<String>,

    /// 修改状态
    #[arg(short = 's', long = "state", value_enum)]
    pub state: Option<PullUpdateState>,
}

/// 执行 pull update 命令
pub async fn run(ctx: &AppContext, args: &UpdateArgs) -> Result<()> {
    if args.title.is_none() && args.body.is_none() && args.state.is_none() {
        bail!("至少需要指定 --title、--body 或 --state 中的一个");
    }

    let client = ctx.api_client()?;

    let req = UpdatePullRequest {
        title: args.title.clone(),
        body: args.body.clone(),
        state: args.state.as_ref().map(ToString::to_string),
    };

    let pull = client.update_pull(&args.number, &req).await?;

    println!(
        "{:<15} {:<15} {:<55} {}",
        format!("#{}", pull.number),
        pull.state,
        pull.title,
        pull.body
    );

    Ok(())
}
