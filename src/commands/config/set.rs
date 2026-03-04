//! cnb config set 子命令（后续实现）

use clap::Args;

/// Set 参数
#[derive(Debug, Args)]
pub struct SetArgs {
    /// 配置项名称
    pub key: String,

    /// 配置项值
    pub value: String,
}
