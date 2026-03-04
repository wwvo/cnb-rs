//! cnb auth login 子命令

use anyhow::Result;
use clap::Args;
use cnb_api::client::CnbClient;
use cnb_core::config::{Config, DEFAULT_SCHEME};
use cnb_core::context::AppContext;

/// 登录参数
#[derive(Debug, Args)]
pub struct LoginArgs {
    /// 直接指定 Token（不提供则交互式输入）
    #[arg(long)]
    pub token: Option<String>,
}

/// 执行登录
pub async fn run(ctx: &AppContext, args: &LoginArgs) -> Result<()> {
    let domain = ctx.domain();

    // 获取 token：优先使用 --token 参数，否则交互式隐藏输入
    let token = match &args.token {
        Some(t) => t.clone(),
        None => {
            eprint!("请输入 Token（输入不会显示）: ");
            rpassword::read_password()?
        }
    };

    let token = token.trim().to_string();
    if token.is_empty() {
        anyhow::bail!("Token 不能为空");
    }

    // 验证 token：用临时客户端调用 /user
    eprintln!("正在验证 Token...");
    let base_url = format!("{DEFAULT_SCHEME}://api.{domain}/");
    let base_web_url = format!("{DEFAULT_SCHEME}://{domain}/");
    let client = CnbClient::new(&base_url, &base_web_url, &token, "")?;

    let user = client.me().await.map_err(|e| {
        anyhow::anyhow!("Token 验证失败: {e}")
    })?;

    // 保存到配置文件
    Config::save_auth(domain, &token, &user.username)?;

    eprintln!("✓ 已登录为 {} ({})", user.username, domain);
    Ok(())
}
