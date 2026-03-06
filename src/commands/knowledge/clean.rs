//! cnb knowledge clean 子命令

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;

/// 清除知识库
#[derive(Debug, Parser)]
pub struct CleanArgs {
    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

pub async fn run(ctx: &AppContext, args: &CleanArgs) -> Result<()> {
    if !confirm_action("确认清除知识库？此操作不可撤销", args.yes)? {
        println!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;

    match client.delete_knowledge_base().await {
        Ok(()) => println!("知识库已清除"),
        Err(cnb_api::error::ApiError::NotFound(msg)) => println!("{msg}"),
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
