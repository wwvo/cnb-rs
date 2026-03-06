//! 配置系统
//!
//! 支持 TOML 配置文件 (`~/.cnb/config.toml`) 和环境变量。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 默认 CNB 域名
pub const DEFAULT_DOMAIN: &str = "cnb.cool";

/// 默认 HTTPS scheme
pub const DEFAULT_SCHEME: &str = "https";

/// 默认 Git 协议
pub const DEFAULT_GIT_PROTOCOL: &str = "https";

/// 配置文件名
pub const CONFIG_FILE: &str = "config.toml";

/// CNB CLI 配置
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// 默认域名
    pub domain: Option<String>,

    /// Git 协议偏好 (https/ssh)
    pub git_protocol: Option<String>,

    /// 认证配置
    pub auth: Option<AuthConfigToml>,
}

/// 认证配置（TOML 层）
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthConfigToml {
    /// 按主机名存储的 token
    #[serde(flatten)]
    pub hosts: std::collections::HashMap<String, HostAuth>,
}

/// 单个主机的认证信息
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HostAuth {
    pub token: Option<String>,
    pub username: Option<String>,
}

impl Config {
    /// 加载配置文件，文件不存在时返回默认配置
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 配置文件路径：~/.cnb/config.toml
    pub fn config_path() -> PathBuf {
        let home = cnb_home_dir();
        home.join(CONFIG_FILE)
    }

    /// 支持的配置 key 列表
    pub const VALID_KEYS: &[&str] = &["domain", "git_protocol"];

    /// 获取配置项的值
    pub fn get_value(&self, key: &str) -> Option<&str> {
        match key {
            "domain" => self.domain.as_deref(),
            "git_protocol" => self.git_protocol.as_deref(),
            _ => None,
        }
    }

    /// 设置配置项的值并写入文件
    pub fn set_value(key: &str, value: &str) -> anyhow::Result<()> {
        let path = Self::config_path();
        let mut config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str::<Config>(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        match key {
            "domain" => config.domain = Some(value.to_string()),
            "git_protocol" => config.git_protocol = Some(value.to_string()),
            _ => anyhow::bail!("未知配置项：{key}\n可用配置项：{}", Self::VALID_KEYS.join(", ")),
        }

        Self::write_config(&path, &config)
    }

    /// 保存认证信息到配置文件
    ///
    /// 保留已有配置，仅更新指定域名的 auth 段。
    pub fn save_auth(domain: &str, token: &str, username: &str) -> anyhow::Result<()> {
        let path = Self::config_path();
        let mut config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str::<Config>(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        let auth = config.auth.get_or_insert_with(AuthConfigToml::default);
        auth.hosts.insert(
            domain.to_string(),
            HostAuth {
                token: Some(token.to_string()),
                username: Some(username.to_string()),
            },
        );

        Self::write_config(&path, &config)
    }

    /// 从配置文件中移除指定域名的认证信息
    pub fn remove_auth(domain: &str) -> anyhow::Result<bool> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(false);
        }

        let content = std::fs::read_to_string(&path)?;
        let mut config = toml::from_str::<Config>(&content).unwrap_or_default();

        let removed = if let Some(auth) = &mut config.auth {
            auth.hosts.remove(domain).is_some()
        } else {
            false
        };

        if removed {
            Self::write_config(&path, &config)?;
        }

        Ok(removed)
    }

    /// 写入配置文件，自动创建父目录
    fn write_config(path: &std::path::Path, config: &Config) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// 获取 CNB 主目录：~/.cnb/
pub fn cnb_home_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cnb")
}
