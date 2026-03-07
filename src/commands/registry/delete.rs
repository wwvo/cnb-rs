//! cnb registry delete 子命令 - 删除制品库

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除制品库
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// 制品库路径
    pub registry: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 registry delete 命令
pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    let client = ctx.api_client()?;

    confirm_action(&format!("确认删除制品库 {}？此操作不可恢复！", args.registry), args.yes)?;

    client.delete_registry(&args.registry).await?;
    success!("制品库 {} 已删除", args.registry);

    Ok(())
}
