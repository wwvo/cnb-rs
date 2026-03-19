//! 徽章相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::{BadgeListResult, BadgeResult, UploadBadgeRequest, UploadBadgeResult};
use std::fmt::Write;
use urlencoding::encode;

impl CnbClient {
    /// 获取指定徽章（JSON 数据）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_badge(
        &self,
        sha: &str,
        badge: &str,
        branch: Option<&str>,
    ) -> Result<BadgeResult, ApiError> {
        let sha = encode(sha);
        let badge_name = format!("{badge}.json");
        let badge_encoded = encode(&badge_name);
        let mut url = format!(
            "{}{}/-/badge/git/{sha}/{badge_encoded}",
            self.base_url, self.repo
        );
        if let Some(b) = branch {
            let _ = write!(url, "?branch={}", encode(b));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取指定徽章（SVG）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response body cannot be read,
    /// or the CNB API returns a non-success status.
    pub async fn get_badge_svg(
        &self,
        sha: &str,
        badge: &str,
        branch: Option<&str>,
    ) -> Result<String, ApiError> {
        let sha = encode(sha);
        let badge_encoded = encode(badge);
        let mut url = format!(
            "{}{}/-/badge/git/{sha}/{badge_encoded}",
            self.base_url, self.repo
        );
        if let Some(b) = branch {
            let _ = write!(url, "?branch={}", encode(b));
        }
        let resp = self.http.get(&url).send().await?;
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            let text = resp.text().await.map_err(ApiError::Network)?;
            return Ok(text);
        }
        Err(Self::map_error_status(status, resp).await)
    }

    /// 列出仓库徽章
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_badges(&self) -> Result<BadgeListResult, ApiError> {
        let url = format!("{}{}/-/badge/list", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 上传自定义徽章
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn upload_badge(
        &self,
        req: &UploadBadgeRequest,
    ) -> Result<UploadBadgeResult, ApiError> {
        let url = format!("{}{}/-/badge/upload", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }
}
