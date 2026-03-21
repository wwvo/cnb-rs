//! 系统凭证存储抽象。
//!
//! 默认实现使用操作系统 keyring，并在每次调用时加上短超时，避免桌面凭证服务
//! 异常时卡住整个 CLI。

use std::sync::mpsc;
use std::time::Duration;

use thiserror::Error;

const KEYRING_TIMEOUT: Duration = Duration::from_secs(3);

/// 凭证存储统一错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CredentialError {
    /// 指定账号的凭证不存在。
    #[error("凭证不存在")]
    NotFound,
    /// 系统凭证存储调用超时。
    #[error("访问系统凭证存储超时")]
    Timeout,
    /// 系统凭证存储可达但调用失败。
    #[error("{0}")]
    Unavailable(String),
}

/// 跨平台凭证存储接口。
pub trait CredentialStore {
    /// 读取凭证。
    fn get(&self, service: &str, account: &str) -> Result<String, CredentialError>;

    /// 写入凭证。
    fn set(&self, service: &str, account: &str, secret: &str) -> Result<(), CredentialError>;

    /// 删除凭证。
    fn delete(&self, service: &str, account: &str) -> Result<(), CredentialError>;
}

/// 默认系统凭证存储。
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemCredentialStore;

impl SystemCredentialStore {
    fn with_timeout<T, F>(operation: F) -> Result<T, CredentialError>
    where
        T: Send + 'static,
        F: FnOnce() -> Result<T, CredentialError> + Send + 'static,
    {
        let (tx, rx) = mpsc::sync_channel(1);

        std::thread::spawn(move || {
            let _ = tx.send(operation());
        });

        match rx.recv_timeout(KEYRING_TIMEOUT) {
            Ok(result) => result,
            Err(mpsc::RecvTimeoutError::Timeout) => Err(CredentialError::Timeout),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(CredentialError::Unavailable(
                "系统凭证存储调用异常中断".to_string(),
            )),
        }
    }
}

impl CredentialStore for SystemCredentialStore {
    fn get(&self, service: &str, account: &str) -> Result<String, CredentialError> {
        let service = service.to_string();
        let account = account.to_string();

        Self::with_timeout(move || platform::get(&service, &account))
    }

    fn set(&self, service: &str, account: &str, secret: &str) -> Result<(), CredentialError> {
        let service = service.to_string();
        let account = account.to_string();
        let secret = secret.to_string();

        Self::with_timeout(move || platform::set(&service, &account, &secret))
    }

    fn delete(&self, service: &str, account: &str) -> Result<(), CredentialError> {
        let service = service.to_string();
        let account = account.to_string();

        Self::with_timeout(move || platform::delete(&service, &account))
    }
}

#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd"
))]
mod platform {
    use super::CredentialError;

    pub fn get(service: &str, account: &str) -> Result<String, CredentialError> {
        let entry = keyring::Entry::new(service, account)
            .map_err(|err| CredentialError::Unavailable(err.to_string()))?;

        match entry.get_password() {
            Ok(secret) => Ok(secret),
            Err(keyring::Error::NoEntry) => Err(CredentialError::NotFound),
            Err(err) => Err(CredentialError::Unavailable(err.to_string())),
        }
    }

    pub fn set(service: &str, account: &str, secret: &str) -> Result<(), CredentialError> {
        let entry = keyring::Entry::new(service, account)
            .map_err(|err| CredentialError::Unavailable(err.to_string()))?;

        entry
            .set_password(secret)
            .map_err(|err| CredentialError::Unavailable(err.to_string()))
    }

    pub fn delete(service: &str, account: &str) -> Result<(), CredentialError> {
        let entry = keyring::Entry::new(service, account)
            .map_err(|err| CredentialError::Unavailable(err.to_string()))?;

        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Err(CredentialError::NotFound),
            Err(err) => Err(CredentialError::Unavailable(err.to_string())),
        }
    }
}

#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd"
)))]
mod platform {
    use super::CredentialError;

    pub fn get(_service: &str, _account: &str) -> Result<String, CredentialError> {
        Err(CredentialError::Unavailable(
            "当前平台不支持系统凭证存储".to_string(),
        ))
    }

    pub fn set(_service: &str, _account: &str, _secret: &str) -> Result<(), CredentialError> {
        Err(CredentialError::Unavailable(
            "当前平台不支持系统凭证存储".to_string(),
        ))
    }

    pub fn delete(_service: &str, _account: &str) -> Result<(), CredentialError> {
        Err(CredentialError::Unavailable(
            "当前平台不支持系统凭证存储".to_string(),
        ))
    }
}
