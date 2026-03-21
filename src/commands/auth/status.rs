//! cnb auth status 子命令

use anyhow::Result;
use cnb_core::auth::{self, AuthLookup, TokenSource};
use cnb_core::context::AppContext;
use cnb_tui::info;

/// 查看当前认证状态
pub async fn run(ctx: &AppContext) -> Result<()> {
    let domain = ctx.domain();
    let config = ctx.config();

    match auth::resolve_auth(domain, config) {
        AuthLookup::Missing => {
            info!("未登录 ({domain})");
            info!("使用 `cnb-rs auth login` 登录，或设置环境变量 CNB_TOKEN");
        }
        AuthLookup::Unavailable(unavailable) => {
            info!("域名：  {domain}");
            info!(
                "用户：  {}",
                unavailable.username.unwrap_or_else(|| "(未知)".to_string())
            );
            info!(
                "Token:  无法读取（来源：{}）",
                describe_source(&unavailable.source)
            );
            info!("状态：  {}", unavailable.message);
        }
        AuthLookup::Found(resolved) => {
            let source_desc = describe_source(&resolved.source);
            let username = resolve_username(ctx, resolved.username.clone()).await;
            let masked = mask_token(&resolved.token);

            info!("域名：  {domain}");
            info!("用户：  {}", username.0);
            info!("Token:  {masked}（来源：{source_desc}）");
            if let Some(status) = username.1 {
                info!("状态：  {status}");
            }
        }
    }

    Ok(())
}

fn describe_source(source: &TokenSource) -> String {
    match source {
        TokenSource::EnvDomain(key) => format!("环境变量 {key}"),
        TokenSource::EnvGeneric => "环境变量 CNB_TOKEN".to_string(),
        TokenSource::Keyring => "系统凭证存储".to_string(),
        TokenSource::ConfigFile => "配置文件 ~/.cnb/config.toml".to_string(),
    }
}

fn mask_token(token: &str) -> String {
    if token.len() > 12 {
        format!("{}****{}", &token[..4], &token[token.len() - 4..])
    } else {
        "****".to_string()
    }
}

async fn resolve_username(
    ctx: &AppContext,
    fallback_username: Option<String>,
) -> (String, Option<String>) {
    match ctx.api_client() {
        Ok(client) => match client.me().await {
            Ok(user) => (user.username, None),
            Err(_) => (
                fallback_username.unwrap_or_else(|| "(未知)".to_string()),
                Some("Token 无效或无法校验".to_string()),
            ),
        },
        Err(_) => (
            fallback_username.unwrap_or_else(|| "(未知)".to_string()),
            Some("无法创建客户端".to_string()),
        ),
    }
}
