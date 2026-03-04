//! CNB API HTTP 客户端

use crate::error::ApiError;
use crate::types::*;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};

/// CNB API 客户端
pub struct CnbClient {
    http: reqwest::Client,
    base_url: String,
    base_web_url: String,
    token: String,
    repo: String,
}

impl CnbClient {
    /// 创建新的 CNB API 客户端
    pub fn new(base_url: &str, base_web_url: &str, token: &str, repo: &str) -> Result<Self, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.cnb.api+json"));
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

        Ok(Self {
            http,
            base_url: base_url.to_string(),
            base_web_url: base_web_url.to_string(),
            token: token.to_string(),
            repo: repo.to_string(),
        })
    }

    /// 获取 API 基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 获取 Web 基础 URL
    pub fn base_web_url(&self) -> &str {
        &self.base_web_url
    }

    /// 获取当前仓库名
    pub fn repo(&self) -> &str {
        &self.repo
    }

    /// 获取 Token
    pub fn token(&self) -> &str {
        &self.token
    }

    /// 获取当前用户信息
    pub async fn me(&self) -> Result<User, ApiError> {
        let url = format!("{}user", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取仓库信息
    pub async fn get_repo(&self) -> Result<Repo, ApiError> {
        let url = format!("{}{}", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    // ==================== Issue API ====================

    /// 获取 Issue 列表
    pub async fn list_issues(&self, opts: &ListIssuesOptions) -> Result<Vec<Issue>, ApiError> {
        let mut url = format!("{}{}/-/issues?page={}&page_size={}&state={}",
            self.base_url, self.repo, opts.page, opts.page_size, opts.state);
        if let Some(ref assignees) = opts.assignees {
            url.push_str(&format!("&assignees={assignees}"));
        }
        if let Some(ref authors) = opts.authors {
            url.push_str(&format!("&authors={authors}"));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取所有 Issue（自动分页）
    pub async fn list_all_issues(&self, state: &str) -> Result<Vec<Issue>, ApiError> {
        let page_size = 100;
        let mut all_issues = Vec::new();
        let mut page = 1u32;

        loop {
            let opts = ListIssuesOptions {
                state: state.to_string(),
                page,
                page_size,
                ..Default::default()
            };
            let issues = self.list_issues(&opts).await?;
            let count = issues.len();
            all_issues.extend(issues);
            if (count as u32) < page_size {
                break;
            }
            page += 1;
        }
        Ok(all_issues)
    }

    /// 获取单个 Issue 详情
    pub async fn get_issue(&self, number: &str) -> Result<IssueDetail, ApiError> {
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 创建 Issue
    pub async fn create_issue(&self, req: &CreateIssueRequest) -> Result<IssueDetail, ApiError> {
        let url = format!("{}{}/-/issues", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新 Issue（用于关闭等操作）
    pub async fn update_issue(&self, number: &str, req: &UpdateIssueRequest) -> Result<(), ApiError> {
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        let status = resp.status().as_u16();
        if status >= 200 && status < 300 {
            return Ok(());
        }
        if status == 401 {
            return Err(ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            ));
        }
        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::HttpStatus { status, body })
    }

    /// 获取 Issue 评论列表
    pub async fn list_issue_comments(&self, number: &str) -> Result<Vec<IssueComment>, ApiError> {
        let url = format!("{}{}/-/issues/{number}/comments?page=1&page_size=100",
            self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 创建 Issue 评论
    pub async fn create_issue_comment(&self, number: &str, req: &CreateCommentRequest) -> Result<(), ApiError> {
        let url = format!("{}{}/-/issues/{number}/comments", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        let status = resp.status().as_u16();
        if status >= 200 && status < 300 {
            return Ok(());
        }
        if status == 401 {
            return Err(ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            ));
        }
        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::HttpStatus { status, body })
    }

    /// 获取 Issue 处理人列表
    pub async fn list_issue_assignees(&self, number: &str) -> Result<Vec<IssueAssignee>, ApiError> {
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 添加 Issue 处理人
    pub async fn add_issue_assignees(&self, number: &str, req: &AddAssigneesRequest) -> Result<(), ApiError> {
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        let status = resp.status().as_u16();
        if status >= 200 && status < 300 {
            return Ok(());
        }
        if status == 401 {
            return Err(ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            ));
        }
        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::HttpStatus { status, body })
    }

    // ==================== Internal ====================

    /// 处理 HTTP 响应，返回反序列化后的结果或错误
    async fn handle_response<T: serde::de::DeserializeOwned>(resp: reqwest::Response) -> Result<T, ApiError> {
        let status = resp.status().as_u16();
        if status >= 200 && status < 300 {
            let data = resp.json::<T>().await?;
            return Ok(data);
        }

        // 401: 认证失败，给出友好提示
        if status == 401 {
            return Err(ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            ));
        }

        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::HttpStatus { status, body })
    }
}
