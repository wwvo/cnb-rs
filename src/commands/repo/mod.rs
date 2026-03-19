//! 仓库管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod archive;
pub mod asset;
pub mod clone;
pub mod contributor;
pub mod create;
pub mod delete;
pub mod edit;
pub mod events;
pub mod fork;
pub mod list;
pub mod pin;
pub mod security;
pub mod settings;
pub mod top_contributors;
pub mod transfer;
pub mod unarchive;
pub mod view;
pub mod visibility;

/// 仓库管理
#[derive(Debug, Parser)]
pub struct RepoCommand {
    #[command(subcommand)]
    pub subcommand: RepoSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum RepoSubcommand {
    /// 归档仓库
    Archive(archive::ArchiveArgs),

    /// 管理仓库资产
    Asset(asset::AssetArgs),

    /// 克隆仓库到本地
    Clone(clone::CloneArgs),

    /// 查看贡献者趋势
    Contributor(contributor::ContributorArgs),

    /// 创建新仓库
    Create(create::CreateArgs),

    /// 删除仓库
    Delete(delete::DeleteArgs),

    /// 编辑仓库信息
    Edit(edit::EditArgs),

    /// 获取仓库动态
    Events(events::EventsArgs),

    /// 查看 Fork 列表
    Fork(fork::ForkArgs),

    /// 列出仓库
    List(list::ListArgs),

    /// 管理仓库墙（置顶仓库）
    Pin(pin::PinArgs),

    /// 查看安全概览
    Security(security::SecurityArgs),

    /// 仓库设置管理
    Settings(settings::SettingsCommand),

    /// 查看活跃用户排名
    TopContributors(top_contributors::TopContributorsArgs),

    /// 转移仓库
    Transfer(transfer::TransferArgs),

    /// 解除仓库归档
    Unarchive(unarchive::UnarchiveArgs),

    /// 查看仓库详情
    View(view::ViewArgs),

    /// 修改仓库可见性
    Visibility(visibility::VisibilityArgs),
}

impl RepoCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            RepoSubcommand::Archive(args) => archive::run(ctx, args).await,
            RepoSubcommand::Asset(args) => asset::run(ctx, args).await,
            RepoSubcommand::Clone(args) => clone::run(ctx, args),
            RepoSubcommand::Contributor(args) => contributor::run(ctx, args).await,
            RepoSubcommand::Create(args) => create::run(ctx, args).await,
            RepoSubcommand::Delete(args) => delete::run(ctx, args).await,
            RepoSubcommand::Edit(args) => edit::run(ctx, args).await,
            RepoSubcommand::Events(args) => events::run(ctx, args).await,
            RepoSubcommand::Fork(args) => fork::run(ctx, args).await,
            RepoSubcommand::List(args) => list::run(ctx, args).await,
            RepoSubcommand::Pin(args) => pin::run(ctx, args).await,
            RepoSubcommand::Security(args) => security::run(ctx, args).await,
            RepoSubcommand::Settings(cmd) => cmd.execute(ctx).await,
            RepoSubcommand::TopContributors(args) => top_contributors::run(ctx, args).await,
            RepoSubcommand::Transfer(args) => transfer::run(ctx, args).await,
            RepoSubcommand::Unarchive(args) => unarchive::run(ctx, args).await,
            RepoSubcommand::View(args) => view::run(ctx, args).await,
            RepoSubcommand::Visibility(args) => visibility::run(ctx, args).await,
        }
    }
}
