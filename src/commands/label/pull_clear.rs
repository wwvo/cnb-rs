//! cnb label pull-clear 子命令 - 清空 Pull 所有标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};

/// 清空 Pull 所有标签
#[derive(Debug, Parser)]
pub struct PullClearArgs {
    /// Pull 编号
    pub number: i64,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 label pull-clear 命令
pub async fn run(ctx: &AppContext, args: &PullClearArgs) -> Result<()> {
    if !confirm_action(&format!("确认清空 Pull #{} 的所有标签？", args.number), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.clear_pull_labels(&args.number.to_string()).await?;

    success!("已清空 Pull #{} 的所有标签", args.number);

    Ok(())
}
