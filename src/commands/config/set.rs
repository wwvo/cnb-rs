//! cnb config set 子命令

use clap::Args;
use cnb_core::config::Config;

/// Set 参数
#[derive(Debug, Args)]
pub struct SetArgs {
    /// 配置项名称
    pub key: String,

    /// 配置项值
    pub value: String,
}

/// 设置配置项的值
pub fn run(args: &SetArgs) -> anyhow::Result<()> {
    Config::set_value(&args.key, &args.value)?;
    eprintln!("✓ {} = {}", args.key, args.value);
    Ok(())
}
