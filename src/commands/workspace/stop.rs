//! cnb workspace stop 子命令 - 停止工作区

use anyhow::{bail, Result};
use clap::Parser;
use cnb_api::types::StopWorkspaceRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 停止工作区
#[derive(Debug, Parser)]
pub struct StopArgs {
    /// 流水线 ID
    #[arg(short = 'p', long = "pipeline-id")]
    pub pipeline_id: Option<String>,

    /// 流水线构建号
    #[arg(long = "sn")]
    pub sn: Option<String>,
}

/// 执行 workspace stop 命令
pub async fn run(ctx: &AppContext, args: &StopArgs) -> Result<()> {
    if args.pipeline_id.is_none() && args.sn.is_none() {
        bail!("请指定 --pipeline-id 或 --sn");
    }

    let client = ctx.api_client()?;

    let req = StopWorkspaceRequest {
        pipeline_id: args.pipeline_id.clone(),
        sn: args.sn.clone(),
    };

    let resp = client.stop_workspace(&req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }

    success!("工作区已停止");
    if !resp.message.is_empty() {
        println!("{}", resp.message);
    }

    Ok(())
}
