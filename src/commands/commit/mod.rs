//! Commit 子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod asset_clean;
pub mod asset_stats;
pub mod asset_upload;

/// Commit 管理
#[derive(Debug, Parser)]
pub struct CommitCommand {
    #[command(subcommand)]
    pub subcommand: CommitSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum CommitSubcommand {
    /// 统计 Commit 附件大小
    #[command(name = "asset-stats")]
    Stats,
    /// 清理 Commit 附件
    #[command(name = "asset-clean")]
    Clean(asset_clean::AssetCleanArgs),
    /// 上传附件到 Commit
    #[command(name = "asset-upload")]
    Upload(asset_upload::AssetUploadArgs),
}

impl CommitCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            CommitSubcommand::Stats => asset_stats::run(ctx).await,
            CommitSubcommand::Clean(args) => asset_clean::run(ctx, args).await,
            CommitSubcommand::Upload(args) => asset_upload::run(ctx, args).await,
        }
    }
}
