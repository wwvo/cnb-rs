//! cnb workspace start 子命令 - 启动工作区

use anyhow::Result;
use clap::Parser;
use cnb_api::types::StartWorkspaceRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 启动工作区
#[derive(Debug, Parser)]
pub struct StartArgs {
    /// 分支名
    #[arg(short = 'b', long = "branch")]
    pub branch: Option<String>,

    /// Tag 名称
    #[arg(short = 't', long = "tag")]
    pub tag: Option<String>,

    /// 自动在浏览器中打开
    #[arg(long)]
    pub open: bool,
}

/// 获取当前 Git 分支名
fn current_git_branch() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;
    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if branch.is_empty() || branch == "HEAD" {
            None
        } else {
            Some(branch)
        }
    } else {
        None
    }
}

/// 执行 workspace start 命令
pub async fn run(ctx: &AppContext, args: &StartArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;

    // 确定分支和 ref
    let (branch, git_ref) = if let Some(ref tag) = args.tag {
        (tag.clone(), format!("refs/tags/{tag}"))
    } else {
        let branch = args
            .branch
            .clone()
            .unwrap_or_else(|| current_git_branch().unwrap_or_else(|| "main".to_string()));
        let git_ref = format!("refs/heads/{branch}");
        (branch, git_ref)
    };

    let req = StartWorkspaceRequest {
        branch,
        git_ref,
    };

    let resp = client.start_workspace(repo, &req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }

    if !resp.url.is_empty() {
        success!("工作区已启动: {}", resp.url);
        if args.open {
            AppContext::open_in_browser(&resp.url)?;
        }
    }

    if !resp.message.is_empty() {
        println!("{}", resp.message);
    }

    Ok(())
}
