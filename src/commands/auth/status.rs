//! cnb auth status 子命令

use anyhow::Result;
use cnb_core::auth::{TokenSource, get_token_with_source};
use cnb_core::context::AppContext;
use cnb_tui::info;

/// 查看当前认证状态
pub async fn run(ctx: &AppContext) -> Result<()> {
    let domain = ctx.domain();
    let config = ctx.config();

    let Some((token, source)) = get_token_with_source(domain, config) else {
        info!("未登录 ({domain})");
        info!("使用 `cnb auth login` 登录，或设置环境变量 CNB_TOKEN");
        return Ok(());
    };

    // Token 脱敏显示：保留前 4 位和后 4 位
    let masked = if token.len() > 12 {
        format!("{}****{}", &token[..4], &token[token.len() - 4..])
    } else {
        "****".to_string()
    };

    let source_desc = match &source {
        TokenSource::EnvDomain(key) => format!("环境变量 {key}"),
        TokenSource::EnvGeneric => "环境变量 CNB_TOKEN".to_string(),
        TokenSource::ConfigFile => "配置文件 ~/.cnb/config.toml".to_string(),
    };

    // 尝试获取用户名
    let username = match ctx.api_client() {
        Ok(client) => match client.me().await {
            Ok(user) => user.username,
            Err(_) => "(Token 无效)".to_string(),
        },
        Err(_) => "(无法创建客户端)".to_string(),
    };

    info!("域名：  {domain}");
    info!("用户：  {username}");
    info!("Token:  {masked}（来源：{source_desc}）");

    Ok(())
}
