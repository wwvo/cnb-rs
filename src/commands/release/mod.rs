//! Release 子命令组

use clap::Parser;

pub mod create;
pub mod list;

/// Release 管理
#[derive(Debug, Parser)]
pub struct ReleaseCommand {
    #[command(subcommand)]
    pub subcommand: ReleaseSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum ReleaseSubcommand {
    /// 列出 Release
    List,
    /// 创建 Release
    Create(create::CreateArgs),
}
