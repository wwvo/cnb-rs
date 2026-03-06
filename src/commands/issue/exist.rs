//! cnb issue exist 子命令 - 检查 Issue 是否存在

use anyhow::Result;
use clap::Parser;
use cnb_api::error::ApiError;
use cnb_core::context::AppContext;

/// 检查 Issue 是否存在
#[derive(Debug, Parser)]
pub struct ExistArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,
}

/// 执行 issue exist 命令
pub async fn run(ctx: &AppContext, args: &ExistArgs) -> Result<()> {
    let client = ctx.api_client()?;

    match client.get_issue(&args.number).await {
        Ok(issue) => println!("{}", issue.title),
        Err(ApiError::NotFound(_)) => println!("false"),
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
