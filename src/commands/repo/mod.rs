//! 仓库管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod create;
pub mod delete;
pub mod edit;
pub mod fork;
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

    /// 删除仓库
    Delete(delete::DeleteArgs),

    /// 编辑仓库信息
    Edit(edit::EditArgs),

    /// 查看 Fork 列表
    Fork(fork::ForkArgs),

    /// 列出仓库
    List(list::ListArgs),

    /// 查看仓库详情
    View(view::ViewArgs),
}

impl RepoCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            RepoSubcommand::Create(args) => create::run(ctx, args).await,
            RepoSubcommand::Delete(args) => delete::run(ctx, args).await,
            RepoSubcommand::Edit(args) => edit::run(ctx, args).await,
            RepoSubcommand::Fork(args) => fork::run(ctx, args).await,
            RepoSubcommand::List(args) => list::run(ctx, args).await,
            RepoSubcommand::View(args) => view::run(ctx, args).await,
        }
    }
}
