//! cnb user activities 子命令 - 查看活动汇总

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查看活动汇总
#[derive(Debug, Parser)]
pub struct ActivitiesArgs {
    /// 用户名（不指定则查看当前用户）
    pub username: Option<String>,

    /// 查询日期（格式 yyyyMM 或 yyyyMMdd，默认当月）
    #[arg(short = 'd', long = "date")]
    pub date: Option<String>,
}

/// 执行 user activities 命令
pub async fn run(ctx: &AppContext, args: &ActivitiesArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let username = match &args.username {
        Some(u) => u.clone(),
        None => client.get_current_user().await?.username,
    };

    let result = client.get_activities(&username, args.date.as_deref()).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    let date_label = args.date.as_deref().unwrap_or("当月");
    eprintln!("{username} {date_label} 活动汇总:");
    println!("  Commits:        {}", result.commit_count);
    println!("  Pull Requests:  {}", result.pull_request_count);
    println!("  Issues:         {}", result.issues_count);
    println!("  Code Reviews:   {}", result.code_review_count);
    println!("  新建仓库:        {}", result.repo_count);
    println!("  新建组织:        {}", result.group_count);

    Ok(())
}
