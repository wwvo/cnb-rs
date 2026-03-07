//! 用户信息子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod activities;
pub mod activity_detail;
pub mod followers;
pub mod following;

/// 用户信息
#[derive(Debug, Parser)]
pub struct UserCommand {
    #[command(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum UserSubcommand {
    /// 查看粉丝列表
    Followers(followers::FollowersArgs),
    /// 查看关注列表
    Following(following::FollowingArgs),
    /// 查看活动汇总
    Activities(activities::ActivitiesArgs),
    /// 查看仓库活动详情
    #[command(name = "activity-detail")]
    ActivityDetail(activity_detail::ActivityDetailArgs),
}

impl UserCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            UserSubcommand::Followers(args) => followers::run(ctx, args).await,
            UserSubcommand::Following(args) => following::run(ctx, args).await,
            UserSubcommand::Activities(args) => activities::run(ctx, args).await,
            UserSubcommand::ActivityDetail(args) => activity_detail::run(ctx, args).await,
        }
    }
}
