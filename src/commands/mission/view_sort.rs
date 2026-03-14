//! cnb mission view sort 子命令 - 排序视图

use anyhow::Result;
use clap::Parser;
use cnb_api::types::MissionView;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 排序任务集视图
#[derive(Debug, Parser)]
pub struct ViewSortArgs {
    /// 任务集路径
    pub mission: String,

    /// 视图 ID 列表（按期望顺序排列，逗号分隔）
    #[arg(long = "ids", value_delimiter = ',')]
    pub ids: Vec<String>,
}

/// 执行 mission view sort 命令
pub async fn run(ctx: &AppContext, args: &ViewSortArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let views: Vec<MissionView> = args
        .ids
        .iter()
        .map(|id| MissionView {
            id: id.clone(),
            name: String::new(),
            view_type: serde_json::Value::Null,
        })
        .collect();

    client.sort_mission_views(&args.mission, &views).await?;
    success!("视图顺序已更新");

    Ok(())
}
