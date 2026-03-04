//! 认证管理
//!
//! Token 解析优先级:
//! 1. 环境变量: CNB_TOKEN_{domain去掉点和横杠} / CNB_TOKEN
//! 2. 配置文件: ~/.cnb/config.toml [auth.{hostname}] token
//! 3. (未来) 系统 keyring

use crate::config::Config;

/// Token 来源
#[derive(Debug)]
pub enum TokenSource {
    /// 域名特定环境变量（如 CNB_TOKEN_cnbcool）
    EnvDomain(String),
    /// 通用环境变量 CNB_TOKEN
    EnvGeneric,
    /// 配置文件 ~/.cnb/config.toml
    ConfigFile,
}

/// 获取指定域名的认证 Token 及其来源
pub fn get_token_with_source(domain: &str, config: &Config) -> Option<(String, TokenSource)> {
    // 优先级 1: 域名特定环境变量
    let env_key = format!(
        "CNB_TOKEN_{}",
        domain.replace('.', "").replace('-', "")
    );
    if let Ok(token) = std::env::var(&env_key) {
        if !token.is_empty() {
            return Some((token, TokenSource::EnvDomain(env_key)));
        }
    }

    // 优先级 2: 通用环境变量
    if let Ok(token) = std::env::var("CNB_TOKEN") {
        if !token.is_empty() {
            return Some((token, TokenSource::EnvGeneric));
        }
    }

    // 优先级 3: 配置文件
    if let Some(auth) = &config.auth {
        if let Some(host_auth) = auth.hosts.get(domain) {
            if let Some(token) = &host_auth.token {
                if !token.is_empty() {
                    return Some((token.clone(), TokenSource::ConfigFile));
                }
            }
        }
    }

    None
}

/// 获取指定域名的认证 Token
///
/// 按优先级依次尝试:
/// 1. 域名特定的环境变量 (如 CNB_TOKEN_cnbcool)
/// 2. 通用环境变量 CNB_TOKEN
/// 3. 配置文件中的 token
pub fn get_token(domain: &str, config: &Config) -> Option<String> {
    get_token_with_source(domain, config).map(|(token, _)| token)
}
