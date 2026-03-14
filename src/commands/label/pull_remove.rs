//! cnb label pull-remove 子命令 - 移除 Pull 单个标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 移除 Pull 单个标签
#[derive(Debug, Parser)]
pub struct PullRemoveArgs {
    /// Pull 编号
    pub number: i64,

    /// 标签名称
    pub name: String,
}

/// 执行 label pull-remove 命令
pub async fn run(ctx: &AppContext, args: &PullRemoveArgs) -> Result<()> {
    let client = ctx.api_client()?;

    client
        .remove_pull_label(&args.number.to_string(), &args.name)
        .await?;

    success!("已从 Pull #{} 移除标签: {}", args.number, args.name);

    Ok(())
}
