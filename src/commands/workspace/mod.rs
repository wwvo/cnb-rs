//! Workspace 云原生工作区子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod closed_clean;
pub mod delete;
pub mod detail;
pub mod list;
pub mod start;
pub mod stop;

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
    /// 列出我的工作区
    List(list::ListArgs),
    /// 启动工作区
    Start(start::StartArgs),
    /// 停止工作区
    Stop(stop::StopArgs),
    /// 删除工作区
    Delete(delete::DeleteArgs),
    /// 查看工作区详情
    Detail(detail::DetailArgs),
}

impl WorkspaceCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            WorkspaceSubcommand::ClosedClean => closed_clean::run(ctx).await,
            WorkspaceSubcommand::List(args) => list::run(ctx, args).await,
            WorkspaceSubcommand::Start(args) => start::run(ctx, args).await,
            WorkspaceSubcommand::Stop(args) => stop::run(ctx, args).await,
            WorkspaceSubcommand::Delete(args) => delete::run(ctx, args).await,
            WorkspaceSubcommand::Detail(args) => detail::run(ctx, args).await,
        }
    }
}
