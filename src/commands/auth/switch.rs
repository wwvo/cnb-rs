//! cnb auth switch 子命令

use std::io;

use anyhow::Result;
use clap::Args;
use cnb_core::auth::{self, AuthAccount, TokenSource};
use cnb_core::context::AppContext;
use cnb_tui::{info, success};

use super::unset_hint;

/// 切换登录账号参数。
#[derive(Debug, Args)]
pub struct SwitchArgs {
    /// 目标用户名；不提供时将从已保存账号中选择
    pub username: Option<String>,
}

/// 执行账号切换。
pub fn run(ctx: &AppContext, args: &SwitchArgs) -> Result<()> {
    let domain = ctx.domain();
    let config = ctx.config();

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
            "当前正在使用环境变量 {key} 进行认证。\n若要切换本地保存的账号，请先清除该环境变量。\n{hint}"
        );
    }

    let accounts = auth::stored_accounts(domain, config);
    if accounts.is_empty() {
        anyhow::bail!("当前域名没有已保存的登录账号：{domain}");
    }

    let current_user = accounts
        .iter()
        .find(|account| account.active)
        .map(|account| account.username.clone());
    let username = match &args.username {
        Some(username) => username.clone(),
        None => select_account(&accounts)?,
    };

    if current_user.as_deref() == Some(username.as_str()) {
        info!("当前已使用账号 {} ({})", username, domain);
        return Ok(());
    }

    if !auth::switch_user(domain, &username)? {
        anyhow::bail!(
            "未找到账号 {}。可用账号：{}",
            username,
            accounts
                .iter()
                .map(|account| account.username.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    success!("已切换到 {} ({})", username, domain);
    Ok(())
}

fn select_account(accounts: &[AuthAccount]) -> Result<String> {
    if accounts.len() == 1 {
        return Ok(accounts[0].username.clone());
    }

    info!("请选择要切换的账号：");
    for (index, account) in accounts.iter().enumerate() {
        let active = if account.active { " (当前)" } else { "" };
        let storage = match account.storage {
            cnb_core::config::AuthTokenStorage::Keyring => "keyring",
            cnb_core::config::AuthTokenStorage::Config => "config",
        };
        info!(
            "  {}. {} [{}]{}",
            index + 1,
            account.username,
            storage,
            active
        );
    }

    eprint!("请输入编号或用户名: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        anyhow::bail!("未选择任何账号");
    }

    if let Ok(index) = input.parse::<usize>()
        && let Some(account) = accounts.get(index.saturating_sub(1))
    {
        return Ok(account.username.clone());
    }

    if let Some(account) = accounts.iter().find(|account| account.username == input) {
        return Ok(account.username.clone());
    }

    anyhow::bail!("无效的账号选择：{input}")
}
