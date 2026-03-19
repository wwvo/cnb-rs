use super::CnbClient;
use crate::error::ApiError;
use crate::types::{Commit, CommitAsset, CommitAssetUploadURL, PostCommitAssetUploadURLRequest};
use urlencoding::encode;

impl CnbClient {
    /// 列出仓库提交。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_commits(&self, page: u32, page_size: u32) -> Result<Vec<Commit>, ApiError> {
        let url = format!(
            "{}{}/-/git/commits?page={page}&page_size={page_size}",
            self.base_url, self.repo
        );
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 列出全部提交（自动分页）。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if any paginated request fails, a response cannot be
    /// deserialized, or the CNB API returns a non-success status.
    pub async fn list_all_commits(&self) -> Result<Vec<Commit>, ApiError> {
        self.paginate(|page, page_size| self.list_commits(page, page_size))
            .await
    }

    /// 获取提交关联资产。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_commit_assets(&self, sha: &str) -> Result<Vec<CommitAsset>, ApiError> {
        let sha = encode(sha);
        let url = format!("{}{}/-/git/commits/{sha}/assets", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 删除提交关联资产。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn delete_commit_asset(&self, sha: &str, asset_id: &str) -> Result<(), ApiError> {
        let sha = encode(sha);
        let asset_id = encode(asset_id);
        let url = format!(
            "{}{}/-/git/commits/{sha}/assets/{asset_id}",
            self.base_url, self.repo
        );
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取提交资产上传地址。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_commit_asset_upload_url(
        &self,
        sha: &str,
        req: &PostCommitAssetUploadURLRequest,
    ) -> Result<CommitAssetUploadURL, ApiError> {
        let sha = encode(sha);
        let url = format!(
            "{}{}/-/git/commits/{sha}/asset-upload-url",
            self.base_url, self.repo
        );
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }
}
