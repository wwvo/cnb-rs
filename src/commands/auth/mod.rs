//! Auth 认证管理子命令

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod login;
pub mod logout;
pub mod status;
pub mod switch;

/// 认证管理
#[derive(Debug, Parser)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub subcommand: AuthSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum AuthSubcommand {
    /// 登录 CNB 平台
    Login(login::LoginArgs),

    /// 查看当前认证状态
    Status,

    /// 切换当前域名的登录账号
    Switch(switch::SwitchArgs),

    /// 退出登录
    Logout,
}

impl AuthCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            AuthSubcommand::Login(args) => login::run(ctx, args).await,
            AuthSubcommand::Status => status::run(ctx).await,
            AuthSubcommand::Switch(args) => switch::run(ctx, args),
            AuthSubcommand::Logout => logout::run(ctx),
        }
    }
}

pub(crate) fn unset_hint(key: &str) -> String {
    #[cfg(windows)]
    {
        format!("请手动执行：$env:{key}=\"\"")
    }

    #[cfg(not(windows))]
    {
        format!("请手动执行：unset {key}")
    }
}
