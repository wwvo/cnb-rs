//! cnb user activity-detail 子命令 - 查看仓库活动详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查看仓库活动详情
#[derive(Debug, Parser)]
pub struct ActivityDetailArgs {
    /// 用户名（不指定则查看当前用户）
    pub username: Option<String>,

    /// 活动类型（commit/pr/issue/review）
    #[arg(short = 't', long = "type")]
    pub activity_type: String,

    /// 仓库路径
    #[arg(short = 'r', long = "repo")]
    pub repo: String,

    /// 查询日期（格式 yyyyMM 或 yyyyMMdd）
    #[arg(short = 'd', long = "date")]
    pub date: String,
}

/// 执行 user activity-detail 命令
pub async fn run(ctx: &AppContext, args: &ActivityDetailArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let username = match &args.username {
        Some(u) => u.clone(),
        None => client.me().await?.username,
    };

    let result = client
        .get_repo_activity_details(&username, &args.activity_type, &args.repo, &args.date)
        .await?;

    // activity-detail 返回的是动态结构，统一 JSON 输出
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
