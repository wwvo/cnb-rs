//! Workspace 云原生工作区子命令组

use clap::Parser;

pub mod closed_clean;

/// 云原生工作区管理
#[derive(Debug, Parser)]
pub struct WorkspaceCommand {
    #[command(subcommand)]
    pub subcommand: WorkspaceSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum WorkspaceSubcommand {
    /// 清理已关闭的工作区
    #[command(name = "closed-clean")]
    ClosedClean,
}
