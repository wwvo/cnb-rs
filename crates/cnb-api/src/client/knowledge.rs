use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn list_knowledge_models(&self) -> Result<Vec<KnowledgeModel>, ApiError> {
        let url = format!("{}{}/-/knowledgebase/models", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn get_knowledge_base_info(&self) -> Result<KnowledgeBaseInfo, ApiError> {
        let url = format!("{}{}/-/knowledgebase", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_knowledge_base(&self) -> Result<(), ApiError> {
        let url = format!("{}{}/-/knowledgebase", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn query_knowledge_base(
        &self, req: &QueryKnowledgeBaseRequest,
    ) -> Result<Vec<KnowledgeQueryResult>, ApiError> {
        let url = format!("{}{}/-/knowledgebase/query", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }
}
