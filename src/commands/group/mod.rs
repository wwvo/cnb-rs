//! Group 组织子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod update_logo;

/// 组织管理
#[derive(Debug, Parser)]
pub struct GroupCommand {
    #[command(subcommand)]
    pub subcommand: GroupSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum GroupSubcommand {
    /// 更新组织 Logo
    #[command(name = "update-logo")]
    UpdateLogo(update_logo::UpdateLogoArgs),
}

impl GroupCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            GroupSubcommand::UpdateLogo(args) => update_logo::run(ctx, args).await,
        }
    }
}
