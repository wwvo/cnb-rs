//! Pull Request 子命令组

use clap::Parser;

pub mod create;
pub mod list;

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
}
