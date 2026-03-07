use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;
use urlencoding::encode;

impl CnbClient {
    pub async fn list_releases(&self, page: u32, page_size: u32) -> Result<Vec<Release>, ApiError> {
        let url = format!("{}{}/-/releases?page={page}&page_size={page_size}",
            self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn list_all_releases(&self) -> Result<Vec<Release>, ApiError> {
        self.paginate(|page, page_size| self.list_releases(page, page_size)).await
    }

    pub async fn get_release_by_tag(&self, repo_name: &str, tag: &str) -> Result<Release, ApiError> {
        let repo_name = Self::encode_path(repo_name);
        let tag = encode(tag);
        let url = format!("{}{repo_name}/-/releases/tag/{tag}", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn create_release(&self, req: &CreateReleaseRequest) -> Result<Release, ApiError> {
        let url = format!("{}{}/-/releases", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_release_asset(&self, release_id: &str, asset_id: &str) -> Result<(), ApiError> {
        let release_id = encode(release_id);
        let asset_id = encode(asset_id);
        let url = format!("{}{}/-/releases/{release_id}/assets/{asset_id}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn get_release_asset_upload_url(
        &self, repo_name: &str, release_id: &str, req: &PostReleaseAssetUploadURLRequest,
    ) -> Result<ReleaseAssetUploadURL, ApiError> {
        let repo_name = Self::encode_path(repo_name);
        let release_id = encode(release_id);
        let url = format!("{}{repo_name}/-/releases/{release_id}/asset-upload-url", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 按 ID 获取 Release
    pub async fn get_release_by_id(&self, release_id: &str) -> Result<Release, ApiError> {
        let release_id = encode(release_id);
        let url = format!("{}{}/-/releases/{release_id}", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取最新 Release
    pub async fn get_latest_release(&self) -> Result<Release, ApiError> {
        let url = format!("{}{}/-/releases/latest", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新 Release
    pub async fn update_release(&self, release_id: &str, req: &UpdateReleaseRequest) -> Result<Release, ApiError> {
        let release_id = encode(release_id);
        let url = format!("{}{}/-/releases/{release_id}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 删除 Release
    pub async fn delete_release(&self, release_id: &str) -> Result<(), ApiError> {
        let release_id = encode(release_id);
        let url = format!("{}{}/-/releases/{release_id}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取 Release 附件下载重定向 URL
    pub async fn get_release_download_url(&self, tag: &str, filename: &str, share: bool) -> Result<String, ApiError> {
        let tag = encode(tag);
        let filename = encode(filename);
        let share_param = if share { "?share=true" } else { "" };
        let url = format!("{}{}/-/releases/download/{tag}/{filename}{share_param}", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        // API 返回 302 重定向，reqwest 默认跟随重定向，最终 URL 即为下载地址
        Ok(resp.url().to_string())
    }
}
