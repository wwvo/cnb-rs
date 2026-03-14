//! cnb repo settings pull-request 子命令 — 合并请求设置

use anyhow::Result;
use clap::Parser;
use cnb_api::types::PullRequestSettingsRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 合并请求设置
#[derive(Debug, Parser)]
pub struct PullRequestCommand {
    #[command(subcommand)]
    pub subcommand: PullRequestSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PullRequestSubcommand {
    /// 查看合并请求设置
    Get(PrGetArgs),
    /// 更新合并请求设置
    Set(PrSetArgs),
}

impl PullRequestCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            PullRequestSubcommand::Get(args) => run_get(ctx, args).await,
            PullRequestSubcommand::Set(args) => run_set(ctx, args).await,
        }
    }
}

/// 查看合并请求设置
#[derive(Debug, Parser)]
pub struct PrGetArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,
}

/// 更新合并请求设置
#[derive(Debug, Parser)]
pub struct PrSetArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 是否允许直接提交合并
    #[arg(long)]
    pub allow_merge_commit: Option<bool>,

    /// 是否允许变基合并
    #[arg(long)]
    pub allow_rebase: Option<bool>,

    /// 是否允许压缩合并
    #[arg(long)]
    pub allow_squash: Option<bool>,

    /// 是否自动添加管理员为评审者
    #[arg(long)]
    pub auto_reviewer: Option<bool>,

    /// 直接合并提交信息风格（default/pull_request_title/pull_request_title_with_body）
    #[arg(long)]
    pub merge_message_style: Option<String>,

    /// 压缩合并提交信息风格（default/pull_request_title/pull_request_title_with_body）
    #[arg(long)]
    pub squash_message_style: Option<String>,
}

fn bool_icon(v: bool) -> &'static str {
    if v { "✓" } else { "✗" }
}

async fn run_get(ctx: &AppContext, args: &PrGetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let settings = client.get_pull_request_settings(repo_path).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&settings)?);
        return Ok(());
    }

    println!("合并请求设置:");
    println!(
        "  允许直接提交合并:       {}  (信息风格: {})",
        bool_icon(settings.allow_merge_commit_merge),
        if settings.merge_commit_message_style.is_empty() {
            "default"
        } else {
            &settings.merge_commit_message_style
        }
    );
    println!(
        "  允许变基合并:           {}",
        bool_icon(settings.allow_rebase_merge)
    );
    println!(
        "  允许压缩合并:           {}  (信息风格: {})",
        bool_icon(settings.allow_squash_merge),
        if settings.squash_commit_message_style.is_empty() {
            "default"
        } else {
            &settings.squash_commit_message_style
        }
    );
    println!(
        "  自动添加管理员为评审者: {}",
        bool_icon(settings.master_auto_as_reviewer)
    );

    Ok(())
}

async fn run_set(ctx: &AppContext, args: &PrSetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let req = PullRequestSettingsRequest {
        allow_merge_commit_merge: args.allow_merge_commit,
        allow_rebase_merge: args.allow_rebase,
        allow_squash_merge: args.allow_squash,
        master_auto_as_reviewer: args.auto_reviewer,
        merge_commit_message_style: args.merge_message_style.clone(),
        squash_commit_message_style: args.squash_message_style.clone(),
    };

    let client = ctx.api_client()?;
    client.set_pull_request_settings(repo_path, &req).await?;

    success!("合并请求设置已更新");

    Ok(())
}
