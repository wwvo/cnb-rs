//! cnb repo settings push-limit 子命令 — 推送限制设置

use anyhow::Result;
use clap::Parser;
use cnb_api::types::PushLimitSettingsRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 推送限制设置
#[derive(Debug, Parser)]
pub struct PushLimitCommand {
    #[command(subcommand)]
    pub subcommand: PushLimitSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PushLimitSubcommand {
    /// 查看推送限制设置
    Get(PlGetArgs),
    /// 更新推送限制设置
    Set(PlSetArgs),
}

impl PushLimitCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            PushLimitSubcommand::Get(args) => run_get(ctx, args).await,
            PushLimitSubcommand::Set(args) => run_set(ctx, args).await,
        }
    }
}

/// 查看推送限制设置
#[derive(Debug, Parser)]
pub struct PlGetArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,
}

/// 更新推送限制设置
#[derive(Debug, Parser)]
pub struct PlSetArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 是否开启单次推送数量限制
    #[arg(long)]
    pub check_push_number: Option<bool>,

    /// 单次推送最大数量
    #[arg(long)]
    pub max_push_number: Option<u32>,

    /// 仅管理员可推送/删除标签和版本
    #[arg(long)]
    pub master_only_tag: Option<bool>,

    /// 提交检查策略：any/registered/pusher
    #[arg(long)]
    pub commit_must_be: Option<String>,
}

fn bool_icon(v: bool) -> &'static str {
    if v { "✓" } else { "✗" }
}

async fn run_get(ctx: &AppContext, args: &PlGetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let settings = client.get_push_limit_settings(repo_path).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&settings)?);
        return Ok(());
    }

    println!("推送限制设置:");
    if settings.check_single_push_number {
        println!(
            "  单次推送数量限制:       {} (最多 {} 个分支/标签)",
            bool_icon(true),
            settings.allow_single_push_number
        );
    } else {
        println!("  单次推送数量限制:       {}", bool_icon(false));
    }
    println!(
        "  仅管理员管理标签/版本:  {}",
        bool_icon(settings.only_master_can_push_tag)
    );
    println!(
        "  提交作者检查:           {} ({})",
        if settings.push_commit_must_be == "any" {
            "不检查"
        } else {
            "已启用"
        },
        if settings.push_commit_must_be.is_empty() {
            "any"
        } else {
            &settings.push_commit_must_be
        }
    );

    Ok(())
}

async fn run_set(ctx: &AppContext, args: &PlSetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let req = PushLimitSettingsRequest {
        check_single_push_number: args.check_push_number,
        allow_single_push_number: args.max_push_number,
        only_master_can_push_tag: args.master_only_tag,
        push_commit_must_be: args.commit_must_be.clone(),
    };

    let client = ctx.api_client()?;
    client.set_push_limit_settings(repo_path, &req).await?;

    success!("推送限制设置已更新");

    Ok(())
}
