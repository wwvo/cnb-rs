//! Build 构建管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod crontab_sync;
pub mod delete_log;
pub mod download_log;
pub mod list;
pub mod stage;
pub mod start;
pub mod status;
pub mod stop;

/// 构建管理
#[derive(Debug, Parser)]
pub struct BuildCommand {
    #[command(subcommand)]
    pub subcommand: BuildSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum BuildSubcommand {
    /// 触发构建
    Start(start::StartArgs),
    /// 停止构建
    Stop(stop::StopArgs),
    /// 查询构建状态
    Status(status::StatusArgs),
    /// 列出构建记录
    List(list::ListArgs),
    /// 查看 Stage 详情
    Stage(stage::StageArgs),
    /// 下载 Runner 日志
    #[command(name = "download-log")]
    DownloadLog(download_log::DownloadLogArgs),
    /// 删除构建日志
    #[command(name = "delete-log")]
    DeleteLog(delete_log::DeleteLogArgs),
    /// 同步定时任务
    #[command(name = "crontab-sync")]
    CrontabSync(crontab_sync::CrontabSyncArgs),
}

impl BuildCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            BuildSubcommand::Start(args) => start::run(ctx, args).await,
            BuildSubcommand::Stop(args) => stop::run(ctx, args).await,
            BuildSubcommand::Status(args) => status::run(ctx, args).await,
            BuildSubcommand::List(args) => list::run(ctx, args).await,
            BuildSubcommand::Stage(args) => stage::run(ctx, args).await,
            BuildSubcommand::DownloadLog(args) => download_log::run(ctx, args).await,
            BuildSubcommand::DeleteLog(args) => delete_log::run(ctx, args).await,
            BuildSubcommand::CrontabSync(args) => crontab_sync::run(ctx, args).await,
        }
    }
}
