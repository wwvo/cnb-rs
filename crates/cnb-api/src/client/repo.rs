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

    /// 创建仓库（POST /{slug}/-/repos）
    pub async fn create_repo(&self, slug: &str, req: &CreateRepoRequest) -> Result<(), ApiError> {
        let url = format!("{}{}/-/repos", self.base_url, slug);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 更新仓库信息（PATCH /{repo}）
    pub async fn update_repo(&self, repo_path: &str, req: &UpdateRepoRequest) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, repo_path);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除仓库（DELETE /{repo}）
    pub async fn delete_repo(&self, repo_path: &str) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, repo_path);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取仓库的 Fork 列表（GET /{repo}/-/forks）
    pub async fn list_forks(&self, repo_path: &str, page: u32, page_size: u32) -> Result<ForkList, ApiError> {
        let url = format!("{}{}/-/forks?page={page}&page_size={page_size}", self.base_url, repo_path);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 归档仓库（POST /{slug}/-/settings/archive）
    pub async fn archive_repo(&self, repo_path: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/archive", self.base_url, repo_path);
        let resp = self.http.post(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 解除仓库归档（POST /{slug}/-/settings/unarchive）
    pub async fn unarchive_repo(&self, repo_path: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/unarchive", self.base_url, repo_path);
        let resp = self.http.post(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置仓库可见性（POST /{repo}/-/settings/set_visibility?visibility=xxx）
    pub async fn set_repo_visibility(&self, repo_path: &str, visibility: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/set_visibility?visibility={}", self.base_url, repo_path, visibility);
        let resp = self.http.post(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 转移仓库（POST /{repo}/-/transfer）
    pub async fn transfer_repo(&self, repo_path: &str, target: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/transfer", self.base_url, repo_path);
        let body = serde_json::json!({
            "source": repo_path,
            "target": target,
        });
        let resp = self.http.post(&url).json(&body).send().await?;
        Self::handle_empty_response(resp).await
    }
}
