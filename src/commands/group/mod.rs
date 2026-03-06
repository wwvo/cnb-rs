//! Group 组织子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod create;
pub mod delete;
pub mod list;
pub mod sub_groups;
pub mod transfer;
pub mod update;
pub mod update_logo;
pub mod view;

/// 组织管理
#[derive(Debug, Parser)]
pub struct GroupCommand {
    #[command(subcommand)]
    pub subcommand: GroupSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum GroupSubcommand {
    /// 列出我的组织
    List(list::ListArgs),
    /// 查看组织详情
    View(view::ViewArgs),
    /// 创建组织
    Create(create::CreateArgs),
    /// 更新组织信息
    Update(update::UpdateArgs),
    /// 删除组织
    Delete(delete::DeleteArgs),
    /// 转移组织
    Transfer(transfer::TransferArgs),
    /// 列出子组织
    #[command(name = "sub-groups")]
    SubGroups(sub_groups::SubGroupsArgs),
    /// 更新组织 Logo
    #[command(name = "update-logo")]
    UpdateLogo(update_logo::UpdateLogoArgs),
}

impl GroupCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            GroupSubcommand::List(args) => list::run(ctx, args).await,
            GroupSubcommand::View(args) => view::run(ctx, args).await,
            GroupSubcommand::Create(args) => create::run(ctx, args).await,
            GroupSubcommand::Update(args) => update::run(ctx, args).await,
            GroupSubcommand::Delete(args) => delete::run(ctx, args).await,
            GroupSubcommand::Transfer(args) => transfer::run(ctx, args).await,
            GroupSubcommand::SubGroups(args) => sub_groups::run(ctx, args).await,
            GroupSubcommand::UpdateLogo(args) => update_logo::run(ctx, args).await,
        }
    }
}
