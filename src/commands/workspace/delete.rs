//! cnb workspace delete 子命令 - 删除工作区

use anyhow::{bail, Result};
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 删除工作区
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// 流水线 ID
    #[arg(short = 'p', long = "pipeline-id")]
    pub pipeline_id: Option<String>,

    /// 流水线构建号
    #[arg(long = "sn")]
    pub sn: Option<String>,
}

/// 执行 workspace delete 命令
pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    let pipeline_id = if let Some(ref id) = args.pipeline_id {
        id.clone()
    } else if args.sn.is_some() {
        // 如果只提供了 sn，需要通过 list 找到对应的 pipeline_id
        // 当前 delete_workspace API 只接受 pipeline_id
        bail!("当前仅支持通过 --pipeline-id 删除工作区");
    } else {
        bail!("请指定 --pipeline-id 或 --sn");
    };

    let client = ctx.api_client()?;
    client.delete_workspace(&pipeline_id).await?;
    success!("工作区已删除");

    Ok(())
}
