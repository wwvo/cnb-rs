//! 标签相关 API

use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;
use urlencoding::encode;

impl CnbClient {
    // ==================== 仓库标签 ====================

    /// 列出仓库标签
    pub async fn list_labels(&self, page: u32, page_size: u32, keyword: Option<&str>) -> Result<Vec<Label>, ApiError> {
        let mut url = format!("{}{}/-/labels?page={page}&page_size={page_size}",
            self.base_url, self.repo);
        if let Some(kw) = keyword {
            url.push_str(&format!("&keyword={}", encode(kw)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 列出仓库所有标签（自动分页）
    pub async fn list_all_labels(&self, keyword: Option<&str>) -> Result<Vec<Label>, ApiError> {
        let kw = keyword.map(String::from);
        self.paginate(|page, page_size| {
            let kw = kw.clone();
            async move {
                self.list_labels(page, page_size, kw.as_deref()).await
            }
        }).await
    }

    /// 创建仓库标签
    pub async fn create_label(&self, req: &CreateLabelRequest) -> Result<Label, ApiError> {
        let url = format!("{}{}/-/labels", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新仓库标签
    pub async fn update_label(&self, name: &str, req: &UpdateLabelRequest) -> Result<Label, ApiError> {
        let name = encode(name);
        let url = format!("{}{}/-/labels/{name}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 删除仓库标签
    pub async fn delete_label(&self, name: &str) -> Result<(), ApiError> {
        let name = encode(name);
        let url = format!("{}{}/-/labels/{name}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    // ==================== Pull 标签 ====================

    /// 列出 Pull 标签
    pub async fn list_pull_labels(&self, number: &str) -> Result<Vec<Label>, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/pulls/{number}/labels", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 添加 Pull 标签
    pub async fn add_pull_labels(&self, number: &str, req: &LabelListRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/pulls/{number}/labels", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置（替换）Pull 标签
    pub async fn set_pull_labels(&self, number: &str, req: &LabelListRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/pulls/{number}/labels", self.base_url, self.repo);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除 Pull 指定标签
    pub async fn remove_pull_label(&self, number: &str, label_name: &str) -> Result<(), ApiError> {
        let number = encode(number);
        let label_name = encode(label_name);
        let url = format!("{}{}/-/pulls/{number}/labels/{label_name}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 清空 Pull 所有标签
    pub async fn clear_pull_labels(&self, number: &str) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/pulls/{number}/labels", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }
}
