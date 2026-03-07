//! cnb build delete-log 子命令 - 删除构建日志

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success, fail};

/// 删除构建日志
#[derive(Debug, Parser)]
pub struct DeleteLogArgs {
    /// 构建号
    pub sn: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 build delete-log 命令
pub async fn run(ctx: &AppContext, args: &DeleteLogArgs) -> Result<()> {
    if !confirm_action(&format!("确认删除构建 {} 的日志？", args.sn), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    let result = client.delete_build_log(&args.sn).await?;

    if result.code == 0 {
        success!("构建日志已删除");
    } else {
        fail!("{}", result.message);
    }

    Ok(())
}
