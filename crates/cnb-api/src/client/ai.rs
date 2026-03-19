use super::CnbClient;
use crate::error::ApiError;
use crate::types::{ChatCompletionsRequest, ChatCompletionsResponse};

impl CnbClient {
    /// 发送 AI Chat 非流式请求。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn ai_chat(
        &self,
        req: &ChatCompletionsRequest,
    ) -> Result<ChatCompletionsResponse, ApiError> {
        let url = format!("{}{}/-/ai/chat/completions", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 发送 AI Chat 流式请求。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn ai_chat_stream(
        &self,
        req: &ChatCompletionsRequest,
    ) -> Result<reqwest::Response, ApiError> {
        let url = format!("{}{}/-/ai/chat/completions", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(resp);
        }
        Err(Self::map_error_status(status, resp).await)
    }
}
