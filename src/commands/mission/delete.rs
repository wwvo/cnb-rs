//! cnb mission delete 子命令 - 删除任务集

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除任务集
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// 任务集路径
    pub mission: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 mission delete 命令
pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    let client = ctx.api_client()?;

    confirm_action(&format!("确认删除任务集 {}？", args.mission), args.yes)?;

    client.delete_mission(&args.mission).await?;
    success!("任务集 {} 已删除", args.mission);

    Ok(())
}
