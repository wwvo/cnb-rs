//! cnb auth logout 子命令

use anyhow::Result;
use cnb_core::auth::{TokenSource, get_token_with_source};
use cnb_core::config::Config;
use cnb_core::context::AppContext;

/// 退出登录
pub async fn run(ctx: &AppContext) -> Result<()> {
    let domain = ctx.domain();
    let config = ctx.config();

    // 检查当前 token 来源
    match get_token_with_source(domain, config) {
        None => {
            eprintln!("未登录 ({domain})");
        }
        Some((_, TokenSource::EnvDomain(key))) => {
            eprintln!("Token 来自环境变量 {key}，无法通过 CLI 移除");
            print_unset_hint(&key);
        }
        Some((_, TokenSource::EnvGeneric)) => {
            eprintln!("Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除");
            print_unset_hint("CNB_TOKEN");
        }
        Some((_, TokenSource::ConfigFile)) => {
            Config::remove_auth(domain)?;
            eprintln!("✓ 已退出 ({domain})");
        }
    }

    Ok(())
}

fn print_unset_hint(key: &str) {
    #[cfg(windows)]
    eprintln!("请手动执行: $env:{key}=\"\"");
    #[cfg(not(windows))]
    eprintln!("请手动执行: unset {key}");
}
