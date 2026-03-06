//! cnb pull create 子命令 - 创建 Pull Request

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreatePullRequest;
use cnb_core::context::AppContext;
use cnb_core::git;

/// 创建 Pull Request
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// 目标分支（默认为仓库的 HEAD 分支）
    #[arg(short = 'B', long = "base-branch")]
    pub base_branch: Option<String>,

    /// 源分支（默认为当前分支）
    #[arg(short = 'H', long = "head-branch")]
    pub head_branch: Option<String>,

    /// 目标仓库路径（默认为当前仓库，用于跨仓库 PR）
    #[arg(short = 'R', long = "remote-repo")]
    pub remote_repo: Option<String>,

    /// PR 标题（默认为最新提交信息）
    #[arg(short = 't', long = "title")]
    pub title: Option<String>,

    /// PR 描述
    #[arg(short = 'b', long = "body", default_value = "")]
    pub body: String,
}

/// 执行 pull create 命令
pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    // 确定目标仓库
    let base_repo = args
        .remote_repo
        .clone()
        .unwrap_or_else(|| client.repo().to_string());

    // 确定目标分支（默认获取远程仓库的 HEAD 分支）
    let base = if let Some(b) = &args.base_branch {
        b.clone()
    } else {
        let head_ref = client.get_head(&base_repo).await?;
        head_ref.name
    };

    // 确定源分支（默认为当前分支）
    let head = if let Some(h) = &args.head_branch {
        h.clone()
    } else {
        git::current_branch()?
    };

    // 确定标题（默认为最新提交标题）
    let title = if let Some(t) = &args.title {
        t.clone()
    } else {
        let (commit_title, _) = git::latest_commit_message()?;
        commit_title
    };

    let req = CreatePullRequest {
        base,
        head,
        head_repo: client.repo().to_string(),
        title,
        body: args.body.clone(),
    };

    let pull = client.create_pull(&base_repo, &req).await?;
    println!(
        "{}{}/-/pulls/{}",
        client.base_web_url(),
        base_repo,
        pull.number
    );

    Ok(())
}
