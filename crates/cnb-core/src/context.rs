//! 应用运行上下文
//!
//! 提供懒加载的配置、API 客户端、Git 信息等共享资源。
//! 借鉴 gh CLI 的 Factory 模式。

use std::sync::OnceLock;

use anyhow::{Result, bail};
use cnb_api::client::CnbClient;

use crate::auth;
use crate::config::{Config, DEFAULT_DOMAIN, DEFAULT_SCHEME};
use crate::git::{self, GitInfo};

/// 应用运行上下文，所有子命令共享
pub struct AppContext {
    /// CLI 传入的 domain 参数
    cli_domain: Option<String>,
    /// CLI 传入的 repo 参数
    cli_repo: Option<String>,
    /// 是否以 JSON 格式输出
    json: bool,

    config: OnceLock<Config>,
    git_info: OnceLock<Option<GitInfo>>,
    api_client: OnceLock<CnbClient>,
}

impl AppContext {
    /// 创建新的上下文
    #[must_use]
    pub fn new(cli_domain: Option<String>, cli_repo: Option<String>, json: bool) -> Self {
        Self {
            cli_domain,
            cli_repo,
            json,
            config: OnceLock::new(),
            git_info: OnceLock::new(),
            api_client: OnceLock::new(),
        }
    }

    /// 是否以 JSON 格式输出
    pub fn json(&self) -> bool {
        self.json
    }

    /// 获取配置（懒加载）
    pub fn config(&self) -> &Config {
        self.config.get_or_init(|| match Config::load() {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("加载配置文件失败，使用默认配置：{e}");
                Config::default()
            }
        })
    }

    /// 获取 Git 信息（懒加载，非 Git 目录返回 None）
    pub fn git_info(&self) -> Option<&GitInfo> {
        self.git_info
            .get_or_init(|| git::parse_git_info_from_current_dir().ok())
            .as_ref()
    }

    /// 获取当前域名（优先 CLI 参数 → Git remote → 默认值）
    pub fn domain(&self) -> &str {
        if let Some(d) = &self.cli_domain {
            return d;
        }
        if let Some(info) = self.git_info() {
            return &info.domain;
        }
        if let Some(d) = &self.config().domain {
            return d;
        }
        DEFAULT_DOMAIN
    }

    /// 获取当前仓库名（优先 CLI 参数 → Git remote）
    ///
    /// # Errors
    ///
    /// Returns an error if the repository cannot be resolved from CLI arguments or
    /// the current Git directory.
    pub fn repo(&self) -> Result<&str> {
        if let Some(r) = &self.cli_repo {
            return Ok(r);
        }
        if let Some(info) = self.git_info() {
            return Ok(&info.repo);
        }
        bail!("无法确定仓库名。请使用 --repo 参数指定，或在 Git 仓库目录下运行。")
    }

    /// 构造仓库的 Web URL（用于浏览器打开）
    ///
    /// `path` 为空时返回仓库首页，否则拼接子路径。
    /// 例：`web_url("")` → `https://cnb.cool/org/repo`
    /// 例：`web_url("-/issues")` → `https://cnb.cool/org/repo/-/issues`
    ///
    /// # Errors
    ///
    /// Returns an error if the repository name cannot be resolved.
    pub fn web_url(&self, path: &str) -> Result<String> {
        let domain = self.domain();
        let scheme = self
            .git_info()
            .map_or(DEFAULT_SCHEME, |i| i.scheme.as_str());
        let repo = self.repo()?;

        if path.is_empty() {
            Ok(format!("{scheme}://{domain}/{repo}"))
        } else {
            Ok(format!("{scheme}://{domain}/{repo}/{path}"))
        }
    }

    /// 在浏览器中打开指定 URL
    ///
    /// # Errors
    ///
    /// Returns an error if the browser cannot be opened.
    pub fn open_in_browser(url: &str) -> Result<()> {
        open::that(url).map_err(|e| anyhow::anyhow!("无法打开浏览器：{e}"))
    }

    /// 获取 API 客户端（懒加载）
    ///
    /// # Errors
    ///
    /// Returns an error if the repository name cannot be resolved or the API client
    /// cannot be initialized.
    pub fn api_client(&self) -> Result<&CnbClient> {
        if let Some(client) = self.api_client.get() {
            return Ok(client);
        }

        let domain = self.domain();
        let scheme = self
            .git_info()
            .map_or(DEFAULT_SCHEME, |i| i.scheme.as_str());

        let base_url = format!("{scheme}://api.{domain}/");
        let base_web_url = format!("{scheme}://{domain}/");
        let token = auth::get_token(domain, self.config()).unwrap_or_default();
        let repo = self.repo().unwrap_or_default().to_string();

        let client = CnbClient::new(&base_url, &base_web_url, &token, &repo)?;

        // OnceLock::set 可能失败（竞争），此时取已有值即可
        let _ = self.api_client.set(client);
        self.api_client
            .get()
            .ok_or_else(|| anyhow::anyhow!("API 客户端初始化失败"))
    }
}
