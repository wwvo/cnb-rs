//! cnb auth logout 子命令

use anyhow::Result;
use cnb_core::auth::{TokenSource, get_token_with_source};
use cnb_core::config::Config;
use cnb_core::context::AppContext;
use cnb_tui::{info, success};

/// 退出登录
pub fn run(ctx: &AppContext) -> Result<()> {
    let domain = ctx.domain();
    let config = ctx.config();

    // 检查当前 token 来源
    match get_token_with_source(domain, config) {
        None => {
            info!("未登录 ({domain})");
        }
        Some((_, TokenSource::EnvDomain(key))) => {
            info!("Token 来自环境变量 {key}，无法通过 CLI 移除");
            print_unset_hint(&key);
        }
        Some((_, TokenSource::EnvGeneric)) => {
            info!("Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除");
            print_unset_hint("CNB_TOKEN");
        }
        Some((_, TokenSource::ConfigFile)) => {
            Config::remove_auth(domain)?;
            success!("已退出 ({domain})");
        }
    }

    Ok(())
}

fn print_unset_hint(key: &str) {
    #[cfg(windows)]
    info!("请手动执行：$env:{key}=\"\"");
    #[cfg(not(windows))]
    info!("请手动执行：unset {key}");
}
