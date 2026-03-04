//! cnb config get 子命令（后续实现）

use clap::Args;

/// Get 参数
#[derive(Debug, Args)]
pub struct GetArgs {
    /// 配置项名称
    pub key: String,
}
