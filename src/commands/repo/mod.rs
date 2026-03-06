//! 仓库管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod create;
pub mod list;
pub mod view;

/// 仓库管理
#[derive(Debug, Parser)]
pub struct RepoCommand {
    #[command(subcommand)]
    pub subcommand: RepoSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum RepoSubcommand {
    /// 创建新仓库
    Create(create::CreateArgs),

    /// 列出仓库
    List(list::ListArgs),

    /// 查看仓库详情
    View(view::ViewArgs),
}

impl RepoCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            RepoSubcommand::Create(args) => create::run(ctx, args).await,
            RepoSubcommand::List(args) => list::run(ctx, args).await,
            RepoSubcommand::View(args) => view::run(ctx, args).await,
        }
    }
}
