//! Config 配置管理子命令

use clap::Parser;

pub mod get;
pub mod list;
pub mod set;

/// 配置管理
#[derive(Debug, Parser)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum ConfigSubcommand {
    /// 列出所有配置项
    List,

    /// 获取配置项的值
    Get(get::GetArgs),

    /// 设置配置项的值
    Set(set::SetArgs),
}
