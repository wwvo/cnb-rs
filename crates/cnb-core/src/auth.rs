//! 认证管理。
//!
//! Token 解析优先级：
//! 1. 环境变量：`CNB_TOKEN_{domain 去掉点和横杠}` / `CNB_TOKEN`
//! 2. 系统 keyring
//! 3. 配置文件：`~/.cnb/config.toml`

use crate::config::{AuthTokenStorage, Config};
use crate::credential_store::{CredentialError, CredentialStore, SystemCredentialStore};

/// Token 来源。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSource {
    /// 域名特定环境变量（如 `CNB_TOKEN_cnbcool`）。
    EnvDomain(String),
    /// 通用环境变量 `CNB_TOKEN`。
    EnvGeneric,
    /// 系统凭证存储。
    Keyring,
    /// 配置文件 `~/.cnb/config.toml`。
    ConfigFile,
}

/// 当前解析到的认证信息。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAuth {
    /// 当前生效的 token。
    pub token: String,
    /// token 来源。
    pub source: TokenSource,
    /// 本地记录的用户名；环境变量来源时通常为空。
    pub username: Option<String>,
}

/// 本地保存了账号元数据，但 token 无法读取。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnavailableStoredAuth {
    /// 预期来源。
    pub source: TokenSource,
    /// 本地记录的用户名。
    pub username: Option<String>,
    /// 失败原因。
    pub message: String,
}

/// 认证解析结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthLookup {
    /// 没有任何认证信息。
    Missing,
    /// 成功解析到可用 token。
    Found(ResolvedAuth),
    /// 找到了本地元数据，但 token 无法读取。
    Unavailable(UnavailableStoredAuth),
}

/// `auth login` 的持久化结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginStorage {
    /// token 已保存到系统凭证存储。
    Keyring,
    /// token 已回退到配置文件。
    ConfigFile,
}

/// `auth login` 结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginResult {
    /// 最终使用的存储位置。
    pub storage: LoginStorage,
    /// 是否因为 keyring 失败而发生了自动回退。
    pub used_fallback: bool,
}

/// 当前域名下保存的账号。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthAccount {
    /// 用户名。
    pub username: String,
    /// Secret 存储位置。
    pub storage: AuthTokenStorage,
    /// 是否为当前激活账号。
    pub active: bool,
}

/// 当前激活且由本地保存的账号。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveStoredAuth {
    /// 用户名。
    pub username: String,
    /// Secret 存储位置。
    pub storage: AuthTokenStorage,
}

/// `auth logout` 结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogoutResult {
    /// 被移除的用户名。
    pub username: String,
    /// 移除后自动切换到的账号。
    pub switched_to: Option<String>,
}

/// 获取指定域名的认证 Token 及其来源。
#[must_use]
pub fn get_token_with_source(domain: &str, config: &Config) -> Option<(String, TokenSource)> {
    match resolve_auth(domain, config) {
        AuthLookup::Found(resolved) => Some((resolved.token, resolved.source)),
        AuthLookup::Missing | AuthLookup::Unavailable(_) => None,
    }
}

/// 获取指定域名的认证 Token。
#[must_use]
pub fn get_token(domain: &str, config: &Config) -> Option<String> {
    get_token_with_source(domain, config).map(|(token, _)| token)
}

/// 解析当前认证状态。
#[must_use]
pub fn resolve_auth(domain: &str, config: &Config) -> AuthLookup {
    let store = SystemCredentialStore;
    resolve_auth_with_store(domain, config, &store)
}

/// 判断当前认证是否来自环境变量。
#[must_use]
pub fn active_env_source(domain: &str) -> Option<TokenSource> {
    env_token(domain).map(|(_, source)| source)
}

/// 列出指定域名下保存的所有账号。
#[must_use]
pub fn stored_accounts(domain: &str, config: &Config) -> Vec<AuthAccount> {
    let Some(host) = config.host_auth(domain) else {
        return Vec::new();
    };

    let active_user = host.active_user();
    host.usernames()
        .into_iter()
        .filter_map(|username| {
            host.user_auth(&username).map(|user_auth| AuthAccount {
                active: active_user == Some(username.as_str()),
                storage: user_auth.storage,
                username,
            })
        })
        .collect()
}

/// 获取当前激活且由本地保存的账号元数据。
#[must_use]
pub fn active_stored_auth(domain: &str, config: &Config) -> Option<ActiveStoredAuth> {
    let host = config.host_auth(domain)?;
    let username = host.active_user()?.to_string();
    let user_auth = host.user_auth(&username)?;

    Some(ActiveStoredAuth {
        username,
        storage: user_auth.storage,
    })
}

/// 持久化登录结果，默认优先使用系统凭证存储。
///
/// # Errors
///
/// Returns an error if the config file cannot be written, or the token cannot be
/// persisted to the selected storage.
pub fn persist_login(
    domain: &str,
    token: &str,
    username: &str,
    insecure_storage: bool,
) -> anyhow::Result<LoginResult> {
    let store = SystemCredentialStore;
    persist_login_with_store(domain, token, username, insecure_storage, &store)
}

/// 切换当前激活账号。
///
/// # Errors
///
/// Returns an error if the target account does not exist, the target secret cannot
/// be read, or the config file cannot be updated.
pub fn switch_user(domain: &str, username: &str) -> anyhow::Result<bool> {
    let store = SystemCredentialStore;
    switch_user_with_store(domain, username, &store)
}

/// 删除当前域名下的激活账号。
///
/// # Errors
///
/// Returns an error if the system credential store cannot be updated or the config
/// file cannot be written.
pub fn logout(domain: &str) -> anyhow::Result<Option<LogoutResult>> {
    let store = SystemCredentialStore;
    logout_with_store(domain, &store)
}

fn resolve_auth_with_store(
    domain: &str,
    config: &Config,
    store: &dyn CredentialStore,
) -> AuthLookup {
    resolve_auth_with_env_lookup(domain, config, store, |key| std::env::var(key).ok())
}

fn resolve_auth_with_env_lookup<F>(
    domain: &str,
    config: &Config,
    store: &dyn CredentialStore,
    env_lookup: F,
) -> AuthLookup
where
    F: Fn(&str) -> Option<String>,
{
    if let Some((token, source)) = env_token_with_lookup(domain, env_lookup) {
        return AuthLookup::Found(ResolvedAuth {
            token,
            source,
            username: None,
        });
    }

    let Some(host) = config.host_auth(domain) else {
        return AuthLookup::Missing;
    };
    let Some(username) = host.active_user().map(ToString::to_string) else {
        return AuthLookup::Missing;
    };
    let Some(user_auth) = host.user_auth(&username) else {
        return AuthLookup::Missing;
    };

    match user_auth.storage {
        AuthTokenStorage::Keyring => match store.get(&keyring_service_name(domain), &username) {
            Ok(token) => AuthLookup::Found(ResolvedAuth {
                token,
                source: TokenSource::Keyring,
                username: Some(username),
            }),
            Err(err) => AuthLookup::Unavailable(UnavailableStoredAuth {
                source: TokenSource::Keyring,
                username: Some(username),
                message: credential_error_message(&err),
            }),
        },
        AuthTokenStorage::Config => {
            let Some(token) = user_auth.token.filter(|token| !token.is_empty()) else {
                return AuthLookup::Unavailable(UnavailableStoredAuth {
                    source: TokenSource::ConfigFile,
                    username: Some(username),
                    message: "配置文件中未找到 token".to_string(),
                });
            };

            AuthLookup::Found(ResolvedAuth {
                token,
                source: TokenSource::ConfigFile,
                username: Some(username),
            })
        }
    }
}

fn persist_login_with_store(
    domain: &str,
    token: &str,
    username: &str,
    insecure_storage: bool,
    store: &dyn CredentialStore,
) -> anyhow::Result<LoginResult> {
    let mut config = Config::load()?;

    if insecure_storage {
        config.upsert_config_auth(domain, username, token);
        config.save()?;

        return Ok(LoginResult {
            storage: LoginStorage::ConfigFile,
            used_fallback: false,
        });
    }

    match store.set(&keyring_service_name(domain), username, token) {
        Ok(()) => {
            config.upsert_keyring_auth(domain, username);
            if let Err(err) = config.save() {
                let rollback = store.delete(&keyring_service_name(domain), username);
                return Err(attach_rollback_error(
                    err.context("写入认证配置失败"),
                    rollback,
                    domain,
                    username,
                ));
            }

            Ok(LoginResult {
                storage: LoginStorage::Keyring,
                used_fallback: false,
            })
        }
        Err(CredentialError::NotFound) => {
            // set 不应出现 NotFound；保守起见仍回退到明文配置。
            config.upsert_config_auth(domain, username, token);
            config.save()?;

            Ok(LoginResult {
                storage: LoginStorage::ConfigFile,
                used_fallback: true,
            })
        }
        Err(CredentialError::Timeout | CredentialError::Unavailable(_)) => {
            config.upsert_config_auth(domain, username, token);
            config.save()?;

            Ok(LoginResult {
                storage: LoginStorage::ConfigFile,
                used_fallback: true,
            })
        }
    }
}

fn switch_user_with_store(
    domain: &str,
    username: &str,
    store: &dyn CredentialStore,
) -> anyhow::Result<bool> {
    let mut config = Config::load()?;
    let Some(host) = config.host_auth(domain) else {
        return Ok(false);
    };
    let Some(user_auth) = host.user_auth(username) else {
        return Ok(false);
    };

    match user_auth.storage {
        AuthTokenStorage::Keyring => {
            store
                .get(&keyring_service_name(domain), username)
                .map(drop)
                .map_err(|err| {
                    anyhow::anyhow!("无法切换到 {username}：{}", credential_error_message(&err))
                })?;
        }
        AuthTokenStorage::Config => {
            if user_auth.token.as_deref().is_none_or(str::is_empty) {
                anyhow::bail!("无法切换到 {username}：配置文件中未找到可用 token");
            }
        }
    }

    if !config.switch_auth_user(domain, username) {
        return Ok(false);
    }

    config.save()?;
    Ok(true)
}

fn logout_with_store(
    domain: &str,
    store: &dyn CredentialStore,
) -> anyhow::Result<Option<LogoutResult>> {
    let mut config = Config::load()?;
    let Some(active_auth) = active_stored_auth(domain, &config) else {
        return Ok(None);
    };

    if active_auth.storage == AuthTokenStorage::Keyring {
        match store.delete(&keyring_service_name(domain), &active_auth.username) {
            Ok(()) | Err(CredentialError::NotFound) => {}
            Err(err) => {
                anyhow::bail!(
                    "删除系统凭证存储中的登录信息失败：{}",
                    credential_error_message(&err)
                );
            }
        }
    }

    if !config.remove_auth_user(domain, &active_auth.username) {
        return Ok(None);
    }

    let switched_to = config
        .host_auth(domain)
        .and_then(|host| host.active_user())
        .map(ToString::to_string);
    config.save()?;

    Ok(Some(LogoutResult {
        username: active_auth.username,
        switched_to,
    }))
}

fn env_token(domain: &str) -> Option<(String, TokenSource)> {
    env_token_with_lookup(domain, |key| std::env::var(key).ok())
}

fn env_token_with_lookup<F>(domain: &str, env_lookup: F) -> Option<(String, TokenSource)>
where
    F: Fn(&str) -> Option<String>,
{
    let env_key = format!("CNB_TOKEN_{}", domain.replace(['.', '-'], ""));
    if let Some(token) = env_lookup(&env_key)
        && !token.is_empty()
    {
        return Some((token, TokenSource::EnvDomain(env_key)));
    }

    if let Some(token) = env_lookup("CNB_TOKEN")
        && !token.is_empty()
    {
        return Some((token, TokenSource::EnvGeneric));
    }

    None
}

fn keyring_service_name(domain: &str) -> String {
    format!("cnb-rs:{domain}")
}

fn credential_error_message(error: &CredentialError) -> String {
    match error {
        CredentialError::NotFound => "系统凭证存储中未找到对应 token".to_string(),
        CredentialError::Timeout => "访问系统凭证存储超时".to_string(),
        CredentialError::Unavailable(message) => message.clone(),
    }
}

fn attach_rollback_error(
    err: anyhow::Error,
    rollback: Result<(), CredentialError>,
    domain: &str,
    username: &str,
) -> anyhow::Error {
    match rollback {
        Ok(()) | Err(CredentialError::NotFound) => err,
        Err(rollback_err) => err.context(format!(
            "回滚系统凭证存储失败（{} / {}）：{}",
            domain,
            username,
            credential_error_message(&rollback_err)
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    use super::*;
    use crate::config::{AuthConfigToml, HostAuth, UserAuth};

    #[derive(Debug, Default)]
    struct MemoryCredentialStore {
        values: Mutex<BTreeMap<(String, String), String>>,
        fail_set: Option<CredentialError>,
        fail_get: Option<CredentialError>,
        fail_delete: Option<CredentialError>,
    }

    impl MemoryCredentialStore {
        fn insert(&self, service: &str, account: &str, secret: &str) {
            let mut values = self
                .values
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            values.insert(
                (service.to_string(), account.to_string()),
                secret.to_string(),
            );
        }
    }

    impl CredentialStore for MemoryCredentialStore {
        fn get(&self, service: &str, account: &str) -> Result<String, CredentialError> {
            if let Some(err) = &self.fail_get {
                return Err(err.clone());
            }

            let values = self
                .values
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            values
                .get(&(service.to_string(), account.to_string()))
                .cloned()
                .ok_or(CredentialError::NotFound)
        }

        fn set(&self, service: &str, account: &str, secret: &str) -> Result<(), CredentialError> {
            if let Some(err) = &self.fail_set {
                return Err(err.clone());
            }

            let mut values = self
                .values
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            values.insert(
                (service.to_string(), account.to_string()),
                secret.to_string(),
            );
            Ok(())
        }

        fn delete(&self, service: &str, account: &str) -> Result<(), CredentialError> {
            if let Some(err) = &self.fail_delete {
                return Err(err.clone());
            }

            let mut values = self
                .values
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if values
                .remove(&(service.to_string(), account.to_string()))
                .is_some()
            {
                Ok(())
            } else {
                Err(CredentialError::NotFound)
            }
        }
    }

    fn config_with_host(domain: &str, host: HostAuth) -> Config {
        Config {
            auth: Some(AuthConfigToml {
                hosts: BTreeMap::from([(domain.to_string(), host)]),
            }),
            ..Config::default()
        }
    }

    fn resolve_auth_without_env(
        domain: &str,
        config: &Config,
        store: &dyn CredentialStore,
    ) -> AuthLookup {
        resolve_auth_with_env_lookup(domain, config, store, |_| None)
    }

    #[test]
    fn resolve_auth_prefers_domain_env() {
        let store = MemoryCredentialStore::default();
        let lookup =
            resolve_auth_with_env_lookup("cnb.cool", &Config::default(), &store, |key| match key {
                "CNB_TOKEN_cnbcool" => Some("domain-token".to_string()),
                "CNB_TOKEN" => Some("generic-token".to_string()),
                _ => None,
            });

        let AuthLookup::Found(resolved) = lookup else {
            panic!("expected token from env");
        };
        assert_eq!(resolved.token, "domain-token");
        assert_eq!(
            resolved.source,
            TokenSource::EnvDomain("CNB_TOKEN_cnbcool".to_string())
        );
    }

    #[test]
    fn resolve_auth_reads_keyring_before_config() {
        let store = MemoryCredentialStore::default();
        store.insert("cnb-rs:cnb.cool", "octocat", "secret-token");
        let config = config_with_host(
            "cnb.cool",
            HostAuth {
                user: Some("octocat".to_string()),
                users: BTreeMap::from([("octocat".to_string(), UserAuth::keyring())]),
                ..HostAuth::default()
            },
        );

        let lookup = resolve_auth_without_env("cnb.cool", &config, &store);
        let AuthLookup::Found(resolved) = lookup else {
            panic!("expected token from keyring");
        };

        assert_eq!(resolved.source, TokenSource::Keyring);
        assert_eq!(resolved.username.as_deref(), Some("octocat"));
        assert_eq!(resolved.token, "secret-token");
    }

    #[test]
    fn resolve_auth_reports_keyring_failure() {
        let store = MemoryCredentialStore {
            fail_get: Some(CredentialError::Unavailable("keyring offline".to_string())),
            ..MemoryCredentialStore::default()
        };
        let config = config_with_host(
            "cnb.cool",
            HostAuth {
                user: Some("octocat".to_string()),
                users: BTreeMap::from([("octocat".to_string(), UserAuth::keyring())]),
                ..HostAuth::default()
            },
        );

        let lookup = resolve_auth_without_env("cnb.cool", &config, &store);
        let AuthLookup::Unavailable(unavailable) = lookup else {
            panic!("expected unavailable result");
        };

        assert_eq!(unavailable.source, TokenSource::Keyring);
        assert_eq!(unavailable.username.as_deref(), Some("octocat"));
        assert!(unavailable.message.contains("keyring offline"));
    }

    #[test]
    fn stored_accounts_supports_legacy_single_user() {
        let config = config_with_host(
            "cnb.cool",
            HostAuth {
                legacy_token: Some("legacy-token".to_string()),
                legacy_username: Some("octocat".to_string()),
                ..HostAuth::default()
            },
        );

        let accounts = stored_accounts("cnb.cool", &config);
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].username, "octocat");
        assert_eq!(accounts[0].storage, AuthTokenStorage::Config);
        assert!(accounts[0].active);
    }
}
