//! cnb auth login 子命令

use std::io::Read;

use anyhow::Result;
use clap::Args;
use cnb_api::client::CnbClient;
use cnb_core::auth::{self, LoginStorage, TokenSource};
use cnb_core::config::DEFAULT_SCHEME;
use cnb_core::context::AppContext;
use cnb_tui::{info, success, warn};

use super::unset_hint;

/// 登录参数
#[derive(Debug, Args)]
pub struct LoginArgs {
    /// 直接指定 Token（不提供则交互式输入）
    #[arg(long, conflicts_with = "with_token")]
    pub token: Option<String>,

    /// 从标准输入读取 Token
    #[arg(long, conflicts_with = "token")]
    pub with_token: bool,

    /// 将认证信息明文保存到配置文件
    #[arg(long)]
    pub insecure_storage: bool,
}

/// 执行登录
pub async fn run(ctx: &AppContext, args: &LoginArgs) -> Result<()> {
    let domain = ctx.domain();

    if let Some(source) = auth::active_env_source(domain) {
        let (key, hint) = match source {
            TokenSource::EnvDomain(key) => {
                let hint = unset_hint(&key);
                (key, hint)
            }
            TokenSource::EnvGeneric => {
                let key = "CNB_TOKEN".to_string();
                let hint = unset_hint(&key);
                (key, hint)
            }
            TokenSource::Keyring | TokenSource::ConfigFile => unreachable!(),
        };

        anyhow::bail!(
            "当前正在使用环境变量 {key} 进行认证。\n若要让 cnb-rs 保存登录信息，请先清除该环境变量。\n{hint}"
        );
    }

    // 获取 token：优先使用 --token 参数，否则交互式隐藏输入
    let token = if let Some(t) = &args.token {
        t.clone()
    } else if args.with_token {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        eprint!("请输入 Token（输入不会显示）: ");
        rpassword::read_password()?
    };

    let token = token.trim().to_string();
    if token.is_empty() {
        anyhow::bail!("Token 不能为空");
    }

    // 验证 token：用临时客户端调用 /user
    info!("正在验证 Token...");
    let base_url = format!("{DEFAULT_SCHEME}://api.{domain}/");
    let base_web_url = format!("{DEFAULT_SCHEME}://{domain}/");
    let client = CnbClient::new(&base_url, &base_web_url, &token, "")?;

    let user = client
        .me()
        .await
        .map_err(|e| anyhow::anyhow!("Token 验证失败：{e}"))?;

    let result = auth::persist_login(domain, &token, &user.username, args.insecure_storage)?;

    match result.storage {
        LoginStorage::Keyring => {}
        LoginStorage::ConfigFile if args.insecure_storage => {
            warn!("认证信息已明文保存到 ~/.cnb/config.toml");
        }
        LoginStorage::ConfigFile if result.used_fallback => {
            warn!("系统凭证存储不可用，认证信息已回退为明文保存到 ~/.cnb/config.toml");
        }
        LoginStorage::ConfigFile => {
            warn!("认证信息已明文保存到 ~/.cnb/config.toml");
        }
    }

    success!("已登录为 {} ({})", user.username, domain);
    Ok(())
}
