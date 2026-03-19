//! 仓库相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::{
    AssetRecord, BranchProtection, BranchProtectionRequest, ContributorTrend, CreateRepoRequest,
    ForkList, ListReposOptions, PipelineSettings, PipelineSettingsRequest, PullRequestSettings,
    PullRequestSettingsRequest, PushLimitSettings, PushLimitSettingsRequest, Repo,
    SecurityOverview, TopContributor, UpdateRepoRequest,
};

impl CnbClient {
    /// 获取指定路径的仓库信息
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_repo_by_path(&self, repo_path: &str) -> Result<Repo, ApiError> {
        let url = format!("{}{}", self.base_url, repo_path);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出当前用户的仓库（GET /user/repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_my_repos(&self, opts: &ListReposOptions) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}user/repos?{}", self.base_url, opts.query_string());
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出指定用户的仓库（GET /users/{username}/repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_user_repos(
        &self,
        username: &str,
        opts: &ListReposOptions,
    ) -> Result<Vec<Repo>, ApiError> {
        let url = format!(
            "{}users/{}/repos?{}",
            self.base_url,
            username,
            opts.query_string()
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出组织的仓库（GET /{slug}/-/repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_group_repos(
        &self,
        slug: &str,
        opts: &ListReposOptions,
    ) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}{}/-/repos?{}", self.base_url, slug, opts.query_string());
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 创建仓库（POST /{slug}/-/repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn create_repo(&self, slug: &str, req: &CreateRepoRequest) -> Result<(), ApiError> {
        let url = format!("{}{}/-/repos", self.base_url, slug);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 更新仓库信息（PATCH /{repo}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn update_repo(
        &self,
        repo_path: &str,
        req: &UpdateRepoRequest,
    ) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, repo_path);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除仓库（DELETE /{repo}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn delete_repo(&self, repo_path: &str) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, repo_path);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取仓库的 Fork 列表（GET /{repo}/-/forks）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_forks(
        &self,
        repo_path: &str,
        page: u32,
        page_size: u32,
    ) -> Result<ForkList, ApiError> {
        let url = format!(
            "{}{}/-/forks?page={page}&page_size={page_size}",
            self.base_url, repo_path
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 归档仓库（POST /{slug}/-/settings/archive）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn archive_repo(&self, repo_path: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/archive", self.base_url, repo_path);
        let resp = self.http.post(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 解除仓库归档（POST /{slug}/-/settings/unarchive）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn unarchive_repo(&self, repo_path: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/unarchive", self.base_url, repo_path);
        let resp = self.http.post(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置仓库可见性（POST /{repo}/-/settings/set_visibility?visibility=xxx）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn set_repo_visibility(
        &self,
        repo_path: &str,
        visibility: &str,
    ) -> Result<(), ApiError> {
        let url = format!(
            "{}{}/-/settings/set_visibility?visibility={}",
            self.base_url, repo_path, visibility
        );
        let resp = self.http.post(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 转移仓库（POST /{repo}/-/transfer）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn transfer_repo(&self, repo_path: &str, target: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/transfer", self.base_url, repo_path);
        let body = serde_json::json!({
            "source": repo_path,
            "target": target,
        });
        let resp = self.http.post(&url).json(&body).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取组织的仓库墙（GET /{slug}/-/pinned-repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_pinned_repos_by_group(&self, slug: &str) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}{}/-/pinned-repos", self.base_url, slug);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取用户的仓库墙（GET /users/{username}/pinned-repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_pinned_repos_by_user(&self, username: &str) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}users/{}/pinned-repos", self.base_url, username);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 设置组织的仓库墙（PUT /{slug}/-/pinned-repos）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn set_pinned_repos_by_group(
        &self,
        slug: &str,
        repos: &[String],
    ) -> Result<Vec<Repo>, ApiError> {
        let url = format!("{}{}/-/pinned-repos", self.base_url, slug);
        let resp = self.http.put(&url).json(&repos).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取贡献者趋势（GET /{slug}/-/contributor/trend）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_contributor_trend(
        &self,
        repo_path: &str,
        limit: u32,
        exclude_external: bool,
    ) -> Result<ContributorTrend, ApiError> {
        let url = format!(
            "{}{}/-/contributor/trend?limit={limit}&exclude_external_users={exclude_external}",
            self.base_url, repo_path
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取安全概览（GET /{repo}/-/security/overview）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_security_overview(
        &self,
        repo_path: &str,
        types: Option<&str>,
        tab: Option<&str>,
    ) -> Result<SecurityOverview, ApiError> {
        let mut url = format!("{}{}/-/security/overview", self.base_url, repo_path);
        let mut params = Vec::new();
        if let Some(t) = types {
            params.push(format!("types={t}"));
        }
        if let Some(t) = tab {
            params.push(format!("tab={t}"));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取活跃用户排名（GET /{repo}/-/top-activity-users）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_top_contributors(
        &self,
        repo_path: &str,
        top: u32,
    ) -> Result<Vec<TopContributor>, ApiError> {
        let url = format!(
            "{}{}/-/top-activity-users?top={top}",
            self.base_url, repo_path
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取仓库动态（GET /events/{repo}/-/{date}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_events(
        &self,
        repo_path: &str,
        date: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!("{}events/{}/-/{}", self.base_url, repo_path, date);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出仓库资产（GET /{slug}/-/list-assets）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_assets(&self, repo_path: &str) -> Result<Vec<AssetRecord>, ApiError> {
        let url = format!("{}{}/-/list-assets", self.base_url, repo_path);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 删除仓库资产（DELETE /{repo}/-/assets/{assetID}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn delete_asset(&self, repo_path: &str, asset_id: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/assets/{}", self.base_url, repo_path, asset_id);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    // ============================
    // 仓库设置 API
    // ============================

    /// 列出分支保护规则（GET /{repo}/-/settings/branch-protections）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_branch_protections(
        &self,
        repo_path: &str,
    ) -> Result<Vec<BranchProtection>, ApiError> {
        let url = format!(
            "{}{}/-/settings/branch-protections",
            self.base_url, repo_path
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取分支保护规则详情（GET /{repo}/-/settings/branch-protections/{id}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_branch_protection(
        &self,
        repo_path: &str,
        id: &str,
    ) -> Result<BranchProtection, ApiError> {
        let url = format!(
            "{}{}/-/settings/branch-protections/{}",
            self.base_url, repo_path, id
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 创建分支保护规则（POST /{repo}/-/settings/branch-protections）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn create_branch_protection(
        &self,
        repo_path: &str,
        req: &BranchProtectionRequest,
    ) -> Result<BranchProtection, ApiError> {
        let url = format!(
            "{}{}/-/settings/branch-protections",
            self.base_url, repo_path
        );
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新分支保护规则（PATCH /{repo}/-/settings/branch-protections/{id}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn update_branch_protection(
        &self,
        repo_path: &str,
        id: &str,
        req: &BranchProtectionRequest,
    ) -> Result<BranchProtection, ApiError> {
        let url = format!(
            "{}{}/-/settings/branch-protections/{}",
            self.base_url, repo_path, id
        );
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 删除分支保护规则（DELETE /{repo}/-/settings/branch-protections/{id}）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn delete_branch_protection(
        &self,
        repo_path: &str,
        id: &str,
    ) -> Result<(), ApiError> {
        let url = format!(
            "{}{}/-/settings/branch-protections/{}",
            self.base_url, repo_path, id
        );
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取合并请求设置（GET /{repo}/-/settings/pull-request）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_pull_request_settings(
        &self,
        repo_path: &str,
    ) -> Result<PullRequestSettings, ApiError> {
        let url = format!("{}{}/-/settings/pull-request", self.base_url, repo_path);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 更新合并请求设置（PUT /{repo}/-/settings/pull-request）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn set_pull_request_settings(
        &self,
        repo_path: &str,
        req: &PullRequestSettingsRequest,
    ) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/pull-request", self.base_url, repo_path);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取推送限制设置（GET /{repo}/-/settings/push-limit）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_push_limit_settings(
        &self,
        repo_path: &str,
    ) -> Result<PushLimitSettings, ApiError> {
        let url = format!("{}{}/-/settings/push-limit", self.base_url, repo_path);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 更新推送限制设置（PUT /{repo}/-/settings/push-limit）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn set_push_limit_settings(
        &self,
        repo_path: &str,
        req: &PushLimitSettingsRequest,
    ) -> Result<(), ApiError> {
        let url = format!("{}{}/-/settings/push-limit", self.base_url, repo_path);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取流水线构建设置（GET /{repo}/-/settings/cloud-native-build）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_pipeline_settings(
        &self,
        repo_path: &str,
    ) -> Result<PipelineSettings, ApiError> {
        let url = format!(
            "{}{}/-/settings/cloud-native-build",
            self.base_url, repo_path
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 更新流水线构建设置（PUT /{repo}/-/settings/cloud-native-build）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn set_pipeline_settings(
        &self,
        repo_path: &str,
        req: &PipelineSettingsRequest,
    ) -> Result<(), ApiError> {
        let url = format!(
            "{}{}/-/settings/cloud-native-build",
            self.base_url, repo_path
        );
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }
}
