//! Pull Request 子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod create;
pub mod list;
pub mod merge;
pub mod update;

/// Pull Request 管理
#[derive(Debug, Parser)]
pub struct PullCommand {
    #[command(subcommand)]
    pub subcommand: PullSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PullSubcommand {
    /// 列出与我相关的 Pull Request
    List,

    /// 创建 Pull Request
    Create(create::CreateArgs),

    /// 更新 Pull Request
    Update(update::UpdateArgs),

    /// 合并 Pull Request
    Merge(merge::MergeArgs),
}

impl PullCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            PullSubcommand::List => list::run(ctx).await,
            PullSubcommand::Create(args) => create::run(ctx, args).await,
            PullSubcommand::Update(args) => update::run(ctx, args).await,
            PullSubcommand::Merge(args) => merge::run(ctx, args).await,
        }
    }
}
