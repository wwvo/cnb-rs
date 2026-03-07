//! Release 子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod asset_clean;
pub mod asset_delete;
pub mod asset_stats;
pub mod asset_upload;
pub mod create;
pub mod delete;
pub mod download;
pub mod latest;
pub mod list;
pub mod update;
pub mod view;

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
    /// 统计 Release 附件大小
    #[command(name = "asset-stats")]
    AssetStats,
    /// 清理 Release 附件
    #[command(name = "asset-clean")]
    AssetClean(asset_clean::AssetCleanArgs),
    /// 上传附件到 Release
    #[command(name = "asset-upload")]
    AssetUpload(asset_upload::AssetUploadArgs),
    /// 查看 Release 详情
    View(view::ViewArgs),
    /// 更新 Release
    Update(update::UpdateArgs),
    /// 删除 Release
    Delete(delete::DeleteArgs),
    /// 查看最新 Release
    Latest,
    /// 下载 Release 附件
    Download(download::DownloadArgs),
    /// 删除单个附件
    #[command(name = "asset-delete")]
    AssetDelete(asset_delete::AssetDeleteArgs),
}

impl ReleaseCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            ReleaseSubcommand::List => list::run(ctx).await,
            ReleaseSubcommand::Create(args) => create::run(ctx, args).await,
            ReleaseSubcommand::AssetStats => asset_stats::run(ctx).await,
            ReleaseSubcommand::AssetClean(args) => asset_clean::run(ctx, args).await,
            ReleaseSubcommand::AssetUpload(args) => asset_upload::run(ctx, args).await,
            ReleaseSubcommand::View(args) => view::run(ctx, args).await,
            ReleaseSubcommand::Update(args) => update::run(ctx, args).await,
            ReleaseSubcommand::Delete(args) => delete::run(ctx, args).await,
            ReleaseSubcommand::Latest => latest::run(ctx).await,
            ReleaseSubcommand::Download(args) => download::run(ctx, args).await,
            ReleaseSubcommand::AssetDelete(args) => asset_delete::run(ctx, args).await,
        }
    }
}
