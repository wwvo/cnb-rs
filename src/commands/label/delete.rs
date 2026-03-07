//! cnb label delete 子命令 - 删除标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};

/// 删除标签
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// 标签名称
    pub name: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 label delete 命令
pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    if !confirm_action(&format!("确认删除标签 {}？", args.name), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.delete_label(&args.name).await?;

    success!("标签 {} 已删除", args.name);

    Ok(())
}
