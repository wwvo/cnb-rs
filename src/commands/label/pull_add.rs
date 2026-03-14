//! cnb label pull-add 子命令 - 为 Pull 添加标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::LabelListRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 为 Pull 添加标签
#[derive(Debug, Parser)]
pub struct PullAddArgs {
    /// Pull 编号
    pub number: i64,

    /// 标签名称（可多次指定）
    #[arg(short = 'l', long = "labels", value_delimiter = ',')]
    pub labels: Vec<String>,
}

/// 执行 label pull-add 命令
pub async fn run(ctx: &AppContext, args: &PullAddArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = LabelListRequest {
        labels: args.labels.clone(),
    };

    client
        .add_pull_labels(&args.number.to_string(), &req)
        .await?;

    success!(
        "已为 Pull #{} 添加标签: {}",
        args.number,
        args.labels.join(", ")
    );

    Ok(())
}
