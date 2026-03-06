//! cnb pull update 子命令 - 更新 Pull Request

use anyhow::{Result, bail};
use clap::Parser;
use cnb_api::types::UpdatePullRequest;
use cnb_core::context::AppContext;

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

    /// 修改状态（open 或 closed）
    #[arg(short = 's', long = "state")]
    pub state: Option<String>,
}

/// 执行 pull update 命令
pub async fn run(ctx: &AppContext, args: &UpdateArgs) -> Result<()> {
    if args.title.is_none() && args.body.is_none() && args.state.is_none() {
        bail!("至少需要指定 --title、--body 或 --state 中的一个");
    }
    if let Some(ref state) = args.state {
        if state != "open" && state != "closed" {
            bail!("--state 只能为 'open' 或 'closed'");
        }
    }

    let client = ctx.api_client()?;

    let req = UpdatePullRequest {
        title: args.title.clone(),
        body: args.body.clone(),
        state: args.state.clone(),
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
