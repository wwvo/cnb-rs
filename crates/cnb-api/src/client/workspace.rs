use super::CnbClient;
use crate::error::ApiError;
use crate::types::*;
use urlencoding::encode;

impl CnbClient {
    pub async fn list_workspaces(
        &self,
        status: &str,
        page: i32,
        page_size: i32,
    ) -> Result<WorkspaceListResponse, ApiError> {
        let mut url = format!(
            "{}user/workspaces?page={page}&page_size={page_size}",
            self.base_url
        );
        if !status.is_empty() {
            url.push_str(&format!("&status={}", encode(status)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 列出工作区（带更多过滤参数）
    pub async fn list_workspaces_with_options(
        &self,
        opts: &ListWorkspacesOptions,
    ) -> Result<WorkspaceListResponse, ApiError> {
        let mut url = format!(
            "{}user/workspaces?page={}&page_size={}",
            self.base_url, opts.page, opts.page_size
        );
        if let Some(ref status) = opts.status {
            url.push_str(&format!("&status={}", encode(status)));
        }
        if let Some(ref slug) = opts.slug {
            url.push_str(&format!("&slug={}", encode(slug)));
        }
        if let Some(ref branch) = opts.branch {
            url.push_str(&format!("&branch={}", encode(branch)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_workspace(&self, pipeline_id: &str) -> Result<(), ApiError> {
        let pipeline_id = encode(pipeline_id);
        let url = format!("{}user/workspaces/{pipeline_id}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 启动工作区
    pub async fn start_workspace(
        &self,
        repo: &str,
        req: &StartWorkspaceRequest,
    ) -> Result<StartWorkspaceResponse, ApiError> {
        let repo = Self::encode_path(repo);
        let url = format!("{}{repo}/-/workspace/start", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 停止工作区
    pub async fn stop_workspace(
        &self,
        req: &StopWorkspaceRequest,
    ) -> Result<StopWorkspaceResponse, ApiError> {
        let url = format!("{}workspace/stop", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取工作区详情
    pub async fn get_workspace_detail(
        &self,
        repo: &str,
        sn: &str,
    ) -> Result<WorkspaceDetailResponse, ApiError> {
        let repo = Self::encode_path(repo);
        let sn = encode(sn);
        let url = format!("{}{repo}/-/workspace/detail/{sn}", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }
}
