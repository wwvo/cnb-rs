//! cnb pull merge 子命令 - 合并 Pull Request

use anyhow::{Result, bail};
use clap::Parser;
use cnb_api::types::MergePullRequestBody;
use cnb_core::context::AppContext;

/// 合并 Pull Request
#[derive(Debug, Parser)]
pub struct MergeArgs {
    /// PR 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 合并提交标题
    #[arg(short = 't', long = "commit-title")]
    pub commit_title: String,

    /// 合并提交信息
    #[arg(short = 'm', long = "commit-message", default_value = "")]
    pub commit_message: String,

    /// 合并方式（merge/squash/rebase）
    #[arg(short = 's', long = "merge-style", default_value = "merge")]
    pub merge_style: String,
}

/// 执行 pull merge 命令
pub async fn run(ctx: &AppContext, args: &MergeArgs) -> Result<()> {
    // 参数校验
    if !["merge", "squash", "rebase"].contains(&args.merge_style.as_str()) {
        bail!("--merge-style 只能为 'merge'、'squash' 或 'rebase'");
    }

    let client = ctx.api_client()?;

    let req = MergePullRequestBody {
        commit_title: args.commit_title.clone(),
        commit_message: args.commit_message.clone(),
        merge_style: args.merge_style.clone(),
    };

    let resp = client.merge_pull(&args.number, &req).await?;

    println!(
        "{:<15} {:<10} {:<15} {:<45} {}",
        format!("#{}", args.number),
        resp.merged,
        args.merge_style,
        resp.sha,
        resp.message
    );

    Ok(())
}
