//! cnb config get 子命令

use clap::Args;
use cnb_core::config::Config;
use cnb_core::context::AppContext;

/// Get 参数
#[derive(Debug, Args)]
pub struct GetArgs {
    /// 配置项名称
    pub key: String,
}

/// 获取配置项的值
pub fn run(ctx: &AppContext, args: &GetArgs) -> anyhow::Result<()> {
    let key = args.key.as_str();

    if !Config::VALID_KEYS.contains(&key) {
        anyhow::bail!(
            "未知配置项: {key}\n可用配置项: {}",
            Config::VALID_KEYS.join(", ")
        );
    }

    let config = ctx.config();
    match config.get_value(key) {
        Some(val) => println!("{val}"),
        None => println!(),
    }

    Ok(())
}
