//! CNB API HTTP 客户端

mod ai;
mod badge;
mod build;
mod commit;
mod gpg_key;
mod group;
mod issue;
mod knowledge;
mod label;
mod member;
mod mission;
mod pull;
mod registry;
mod release;
mod repo;
mod star;
mod user;
mod workspace;

use crate::error::ApiError;
use crate::types::{Content, Repo, User};
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use urlencoding::encode;

/// CNB API 客户端
pub struct CnbClient {
    pub(crate) http: reqwest::Client,
    pub(crate) http_plain: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) base_web_url: String,
    pub(crate) token: String,
    pub(crate) repo: String,
}

impl CnbClient {
    /// 创建新的 CNB API 客户端
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the authorization header is invalid or the underlying
    /// HTTP client cannot be constructed.
    pub fn new(
        base_url: &str,
        base_web_url: &str,
        token: &str,
        repo: &str,
    ) -> Result<Self, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.cnb.api+json"),
        );
        if !token.is_empty() {
            let auth_value = format!("Bearer {token}");
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value).map_err(|e| ApiError::Auth(e.to_string()))?,
            );
        }

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let http_plain = reqwest::Client::new();

        Ok(Self {
            http,
            http_plain,
            base_url: base_url.to_string(),
            base_web_url: base_web_url.to_string(),
            token: token.to_string(),
            repo: repo.to_string(),
        })
    }

    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    #[must_use]
    pub fn base_web_url(&self) -> &str {
        &self.base_web_url
    }
    #[must_use]
    pub fn repo(&self) -> &str {
        &self.repo
    }
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// 获取无默认认证头的 HTTP 客户端引用（用于外部模块复用连接池）
    #[must_use]
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_plain
    }

    /// 获取当前用户信息
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn me(&self) -> Result<User, ApiError> {
        let url = format!("{}user", self.base_url);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取仓库信息
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_repo(&self) -> Result<Repo, ApiError> {
        let url = format!("{}{}", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取仓库文件/目录内容
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_content(&self, path: &str, git_ref: &str) -> Result<Content, ApiError> {
        let path = Self::encode_path(path);
        let git_ref = encode(git_ref);
        let url = format!(
            "{}{}/-/git/contents/{path}?ref={git_ref}",
            self.base_url, self.repo
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    // ==================== Internal ====================

    /// 发送 **读操作** 请求并在 5xx / 网络错误时自动重试（最多 3 次，退避 100ms → 500ms → 1s）
    ///
    /// 4xx 错误不重试（客户端错误重试无意义）。
    /// `build` 闭包每次重试时重新构建 `RequestBuilder`。
    ///
    /// # 重试策略设计说明
    ///
    /// 此方法仅用于 **幂等的读操作**（GET 请求）。写操作（POST/PUT/PATCH/DELETE）
    /// 直接调用 `.send()` 而不经过此方法，因为写操作通常不是幂等的——自动重试
    /// 可能导致资源重复创建、重复删除等副作用。各写操作方法中不使用
    /// `send_with_retry` 是刻意为之，而非遗漏。
    pub(crate) async fn send_with_retry<F>(&self, build: F) -> Result<reqwest::Response, ApiError>
    where
        F: Fn() -> reqwest::RequestBuilder,
    {
        const MAX_RETRIES: u32 = 3;
        const DELAYS_MS: [u64; 3] = [100, 500, 1000];

        let mut last_err: Option<ApiError> = None;

        for attempt in 0..=MAX_RETRIES {
            if attempt > 0 {
                let delay = DELAYS_MS
                    .get((attempt - 1) as usize)
                    .copied()
                    .unwrap_or(1000);
                tracing::warn!("请求失败，{delay}ms 后第 {attempt} 次重试...");
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            }

            match build().send().await {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    // 5xx 可重试
                    if status >= 500 && attempt < MAX_RETRIES {
                        let body = resp.text().await.unwrap_or_default();
                        last_err = Some(ApiError::HttpStatus { status, body });
                        continue;
                    }
                    return Ok(resp);
                }
                // 网络错误可重试（排除请求构建错误）
                Err(e) if attempt < MAX_RETRIES && !e.is_builder() => {
                    last_err = Some(ApiError::Network(e));
                }
                Err(e) => return Err(ApiError::Network(e)),
            }
        }

        Err(last_err.unwrap_or_else(|| ApiError::Api("重试次数用尽".to_string())))
    }

    /// 对含 `/` 的路径做 URL 编码，保留 `/` 分隔符
    pub(crate) fn encode_path(path: &str) -> String {
        path.split('/')
            .map(|seg| encode(seg))
            .collect::<Vec<_>>()
            .join("/")
    }

    pub(crate) async fn paginate<T, F, Fut>(&self, fetch: F) -> Result<Vec<T>, ApiError>
    where
        F: Fn(u32, u32) -> Fut,
        Fut: std::future::Future<Output = Result<Vec<T>, ApiError>>,
    {
        const PAGE_SIZE_U32: u32 = 100;
        const PAGE_SIZE_USIZE: usize = 100;
        let mut all = Vec::new();
        let mut page = 1u32;
        loop {
            let items = fetch(page, PAGE_SIZE_U32).await?;
            let count = items.len();
            all.extend(items);
            if count < PAGE_SIZE_USIZE {
                break;
            }
            page += 1;
        }
        Ok(all)
    }

    pub(crate) async fn handle_empty_response(resp: reqwest::Response) -> Result<(), ApiError> {
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(());
        }
        Err(Self::map_error_status(status, resp).await)
    }

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, ApiError> {
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            let data = resp.json::<T>().await?;
            return Ok(data);
        }
        Err(Self::map_error_status(status, resp).await)
    }

    /// 将非成功 HTTP 状态码映射为对应的 `ApiError`
    async fn map_error_status(status: u16, resp: reqwest::Response) -> ApiError {
        if status == 401 {
            return ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            );
        }
        let body = resp.text().await.unwrap_or_default();
        if status == 404 {
            return ApiError::NotFound(body);
        }
        ApiError::HttpStatus { status, body }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_path_simple() {
        assert_eq!(CnbClient::encode_path("src/main.rs"), "src/main.rs");
    }

    #[test]
    fn encode_path_with_special_chars() {
        // 空格和中文应被编码，`/` 保留
        let encoded = CnbClient::encode_path("docs/我的文件 1.md");
        assert!(encoded.contains('/'));
        assert!(!encoded.contains(' '));
        assert!(!encoded.contains('我'));
    }

    #[test]
    fn encode_path_single_segment() {
        assert_eq!(CnbClient::encode_path("README.md"), "README.md");
    }

    #[test]
    fn encode_path_empty() {
        assert_eq!(CnbClient::encode_path(""), "");
    }

    #[test]
    fn encode_path_preserves_slashes() {
        let encoded = CnbClient::encode_path("a/b/c/d");
        assert_eq!(encoded, "a/b/c/d");
    }

    #[test]
    fn new_client_with_token() {
        let client = CnbClient::new(
            "https://api.cnb.cool/",
            "https://cnb.cool/",
            "test_token",
            "org/repo",
        );
        assert!(client.is_ok());
        let client = client.unwrap_or_else(|e| panic!("创建客户端失败：{e}"));
        assert_eq!(client.base_url(), "https://api.cnb.cool/");
        assert_eq!(client.base_web_url(), "https://cnb.cool/");
        assert_eq!(client.repo(), "org/repo");
        assert_eq!(client.token(), "test_token");
    }

    #[test]
    fn new_client_without_token() {
        let client = CnbClient::new("https://api.cnb.cool/", "https://cnb.cool/", "", "org/repo");
        assert!(client.is_ok());
    }
}
