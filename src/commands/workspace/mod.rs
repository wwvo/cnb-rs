//! Workspace 云原生工作区子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

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

impl WorkspaceCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            WorkspaceSubcommand::ClosedClean => closed_clean::run(ctx).await,
        }
    }
}
