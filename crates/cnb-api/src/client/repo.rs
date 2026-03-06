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
}
