//! GPG 密钥相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::GpgPublicKey;
use std::fmt::Write;
use urlencoding::encode;

impl CnbClient {
    /// 列出 GPG 密钥
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_gpg_keys(
        &self,
        keyword: Option<&str>,
    ) -> Result<Vec<GpgPublicKey>, ApiError> {
        let mut url = format!("{}/user/gpg-keys?page=1&page_size=100", self.base_url);
        if let Some(kw) = keyword {
            let _ = write!(url, "&keyword={}", encode(kw));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }
}
