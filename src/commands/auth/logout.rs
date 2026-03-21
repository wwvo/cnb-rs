//! cnb auth logout 子命令

use anyhow::Result;
use cnb_core::auth::{self, TokenSource};
use cnb_core::context::AppContext;
use cnb_tui::{info, success};

use super::unset_hint;

/// 退出登录
pub fn run(ctx: &AppContext) -> Result<()> {
    let domain = ctx.domain();
    let config = ctx.config();

    match auth::active_env_source(domain) {
        Some(TokenSource::EnvDomain(key)) => {
            info!("Token 来自环境变量 {key}，无法通过 CLI 移除");
            info!("{}", unset_hint(&key));
        }
        Some(TokenSource::EnvGeneric) => {
            info!("Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除");
            info!("{}", unset_hint("CNB_TOKEN"));
        }
        Some(TokenSource::Keyring | TokenSource::ConfigFile) => unreachable!(),
        None => match auth::logout(domain)? {
            None => {
                let has_saved_accounts = !auth::stored_accounts(domain, config).is_empty();
                if has_saved_accounts {
                    info!("未找到可退出的激活账号 ({domain})");
                } else {
                    info!("未登录 ({domain})");
                }
            }
            Some(result) => {
                if let Some(switched_to) = result.switched_to {
                    success!(
                        "已退出 {} ({})，当前账号已切换为 {}",
                        result.username,
                        domain,
                        switched_to
                    );
                } else {
                    success!("已退出 {} ({})", result.username, domain);
                }
            }
        },
    }

    Ok(())
}
