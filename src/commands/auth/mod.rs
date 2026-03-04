//! Auth 认证管理子命令

use clap::Parser;

pub mod login;
pub mod logout;
pub mod status;

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

    /// 退出登录
    Logout,
}
