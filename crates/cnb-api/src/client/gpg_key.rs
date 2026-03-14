//! GPG 密钥相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::GpgPublicKey;
use urlencoding::encode;

impl CnbClient {
    /// 列出 GPG 密钥
    pub async fn list_gpg_keys(
        &self,
        keyword: Option<&str>,
    ) -> Result<Vec<GpgPublicKey>, ApiError> {
        let mut url = format!("{}/user/gpg-keys?page=1&page_size=100", self.base_url);
        if let Some(kw) = keyword {
            url.push_str(&format!("&keyword={}", encode(kw)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }
}
