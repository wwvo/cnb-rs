//! cnb build crontab-sync 子命令 - 同步定时任务

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{success, fail};

/// 同步定时任务
#[derive(Debug, Parser)]
pub struct CrontabSyncArgs {
    /// 分支名（默认 main）
    #[arg(default_value = "main")]
    pub branch: String,
}

/// 执行 build crontab-sync 命令
pub async fn run(ctx: &AppContext, args: &CrontabSyncArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let result = client.build_crontab_sync(&args.branch).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    if result.code == 0 {
        success!("分支 {} 的定时任务已同步", args.branch);
    } else {
        fail!("{}", result.message);
    }

    Ok(())
}
