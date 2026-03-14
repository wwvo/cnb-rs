//! cnb repo settings pipeline 子命令 — 流水线构建设置

use anyhow::Result;
use clap::Parser;
use cnb_api::types::PipelineSettingsRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 流水线构建设置
#[derive(Debug, Parser)]
pub struct PipelineCommand {
    #[command(subcommand)]
    pub subcommand: PipelineSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PipelineSubcommand {
    /// 查看流水线构建设置
    Get(PipeGetArgs),
    /// 更新流水线构建设置
    Set(PipeSetArgs),
}

impl PipelineCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            PipelineSubcommand::Get(args) => run_get(ctx, args).await,
            PipelineSubcommand::Set(args) => run_set(ctx, args).await,
        }
    }
}

/// 查看流水线构建设置
#[derive(Debug, Parser)]
pub struct PipeGetArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,
}

/// 更新流水线构建设置
#[derive(Debug, Parser)]
pub struct PipeSetArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 是否按 .cnb.yml 自动触发构建
    #[arg(long)]
    pub auto_trigger: Option<bool>,

    /// 是否允许 Fork 仓库自动触发构建
    #[arg(long)]
    pub fork_auto_trigger: Option<bool>,
}

fn bool_icon(v: bool) -> &'static str {
    if v { "✓" } else { "✗" }
}

async fn run_get(ctx: &AppContext, args: &PipeGetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let settings = client.get_pipeline_settings(repo_path).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&settings)?);
        return Ok(());
    }

    println!("流水线构建设置:");
    println!(
        "  自动触发构建:           {}",
        bool_icon(settings.auto_trigger)
    );
    println!(
        "  Fork 仓库自动触发:      {}",
        bool_icon(settings.forked_repo_auto_trigger)
    );

    Ok(())
}

async fn run_set(ctx: &AppContext, args: &PipeSetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let req = PipelineSettingsRequest {
        auto_trigger: args.auto_trigger,
        forked_repo_auto_trigger: args.fork_auto_trigger,
    };

    let client = ctx.api_client()?;
    client.set_pipeline_settings(repo_path, &req).await?;

    success!("流水线构建设置已更新");

    Ok(())
}
