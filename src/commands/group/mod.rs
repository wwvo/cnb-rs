//! Group 组织子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod list;
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
    /// 更新组织 Logo
    #[command(name = "update-logo")]
    UpdateLogo(update_logo::UpdateLogoArgs),
}

impl GroupCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            GroupSubcommand::List(args) => list::run(ctx, args).await,
            GroupSubcommand::View(args) => view::run(ctx, args).await,
            GroupSubcommand::UpdateLogo(args) => update_logo::run(ctx, args).await,
        }
    }
}
