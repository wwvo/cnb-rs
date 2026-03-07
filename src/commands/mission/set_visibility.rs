//! cnb mission set-visibility 子命令 - 设置任务集可见性

use anyhow::Result;
use clap::Parser;
use cnb_api::types::SetMissionVisibilityRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 设置任务集可见性
#[derive(Debug, Parser)]
pub struct SetVisibilityArgs {
    /// 任务集路径
    pub mission: String,

    /// 可见性（public/private/secret）
    pub visibility: String,
}

/// 执行 mission set-visibility 命令
pub async fn run(ctx: &AppContext, args: &SetVisibilityArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = SetMissionVisibilityRequest {
        visibility: args.visibility.clone(),
    };

    client.set_mission_visibility(&args.mission, &req).await?;
    success!("任务集可见性已设置为 {}", args.visibility);

    Ok(())
}
