//! cnb label pull-set 子命令 - 替换 Pull 标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::LabelListRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 替换 Pull 标签
#[derive(Debug, Parser)]
pub struct PullSetArgs {
    /// Pull 编号
    pub number: i64,

    /// 新标签列表（可多次指定）
    #[arg(short = 'l', long = "labels", value_delimiter = ',')]
    pub labels: Vec<String>,
}

/// 执行 label pull-set 命令
pub async fn run(ctx: &AppContext, args: &PullSetArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = LabelListRequest {
        labels: args.labels.clone(),
    };

    client
        .set_pull_labels(&args.number.to_string(), &req)
        .await?;

    success!(
        "已替换 Pull #{} 的标签为: {}",
        args.number,
        args.labels.join(", ")
    );

    Ok(())
}
