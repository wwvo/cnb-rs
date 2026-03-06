//! 仓库相关 API

use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    /// 获取指定路径的仓库信息
    pub async fn get_repo_by_path(&self, repo_path: &str) -> Result<Repo, ApiError> {
        let url = format!("{}{}", self.base_url, repo_path);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出当前用户的仓库（GET /user/repos）
    pub async fn list_my_repos(&self, opts: &ListReposOptions) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}user/repos?{}", self.base_url, opts.query_string());
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出指定用户的仓库（GET /users/{username}/repos）
    pub async fn list_user_repos(&self, username: &str, opts: &ListReposOptions) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}users/{}/repos?{}", self.base_url, username, opts.query_string());
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出组织的仓库（GET /{slug}/-/repos）
    pub async fn list_group_repos(&self, slug: &str, opts: &ListReposOptions) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}{}/-/repos?{}", self.base_url, slug, opts.query_string());
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }
}
