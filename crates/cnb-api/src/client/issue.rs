use super::CnbClient;
use crate::error::ApiError;
use crate::types::{
    AddAssigneesRequest, CreateCommentRequest, CreateIssueRequest, Issue, IssueAssignee,
    IssueComment, IssueDetail, IssueLabel, IssueLabelRequest, ListIssuesOptions,
    UpdateIssueRequest,
};
use std::fmt::Write;
use urlencoding::encode;

impl CnbClient {
    /// 列出仓库 Issue。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_issues(&self, opts: &ListIssuesOptions) -> Result<Vec<Issue>, ApiError> {
        let mut url = format!(
            "{}{}/-/issues?page={}&page_size={}&state={}",
            self.base_url,
            self.repo,
            opts.page,
            opts.page_size,
            encode(&opts.state)
        );
        if let Some(ref keyword) = opts.keyword {
            let _ = write!(url, "&keyword={}", encode(keyword));
        }
        if let Some(ref priority) = opts.priority {
            let _ = write!(url, "&priority={}", encode(priority));
        }
        if let Some(ref labels) = opts.labels {
            let _ = write!(url, "&labels={}", encode(labels));
        }
        if let Some(ref labels_operator) = opts.labels_operator {
            let _ = write!(url, "&labels_operator={}", encode(labels_operator));
        }
        if let Some(ref authors) = opts.authors {
            let _ = write!(url, "&authors={}", encode(authors));
        }
        if let Some(ref assignees) = opts.assignees {
            let _ = write!(url, "&assignees={}", encode(assignees));
        }
        if let Some(ref begin) = opts.updated_time_begin {
            let _ = write!(url, "&updated_time_begin={}", encode(begin));
        }
        if let Some(ref end) = opts.updated_time_end {
            let _ = write!(url, "&updated_time_end={}", encode(end));
        }
        if let Some(ref begin) = opts.close_time_begin {
            let _ = write!(url, "&close_time_begin={}", encode(begin));
        }
        if let Some(ref end) = opts.close_time_end {
            let _ = write!(url, "&close_time_end={}", encode(end));
        }
        if let Some(ref order_by) = opts.order_by {
            let _ = write!(url, "&order_by={}", encode(order_by));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出仓库全部 Issue（自动分页）。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if any paginated request fails, a response cannot be
    /// deserialized, or the CNB API returns a non-success status.
    pub async fn list_all_issues(&self, state: &str) -> Result<Vec<Issue>, ApiError> {
        self.paginate(|page, page_size| async move {
            let opts = ListIssuesOptions {
                state: state.to_string(),
                page,
                page_size,
                ..Default::default()
            };
            self.list_issues(&opts).await
        })
        .await
    }

    /// 使用指定筛选条件列出仓库全部 Issue（自动分页）。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if any paginated request fails, a response cannot be
    /// deserialized, or the CNB API returns a non-success status.
    pub async fn list_all_issues_with_options(
        &self,
        opts: &ListIssuesOptions,
    ) -> Result<Vec<Issue>, ApiError> {
        self.paginate(|page, page_size| {
            let mut opts = opts.clone();
            async move {
                opts.page = page;
                opts.page_size = page_size;
                self.list_issues(&opts).await
            }
        })
        .await
    }

    /// 获取单个 Issue 的详情。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_issue(&self, number: &str) -> Result<IssueDetail, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 创建 Issue。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn create_issue(&self, req: &CreateIssueRequest) -> Result<IssueDetail, ApiError> {
        let url = format!("{}{}/-/issues", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新 Issue。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn update_issue(
        &self,
        number: &str,
        req: &UpdateIssueRequest,
    ) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出 Issue 评论。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_issue_comments(
        &self,
        number: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<IssueComment>, ApiError> {
        let number = encode(number);
        let url = format!(
            "{}{}/-/issues/{number}/comments?page={page}&page_size={page_size}",
            self.base_url, self.repo
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取全部 Issue 评论（自动分页）
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if any paginated request fails, a response cannot be
    /// deserialized, or the CNB API returns a non-success status.
    pub async fn list_all_issue_comments(
        &self,
        number: &str,
    ) -> Result<Vec<IssueComment>, ApiError> {
        self.paginate(|page, page_size| self.list_issue_comments(number, page, page_size))
            .await
    }

    /// 创建 Issue 评论。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn create_issue_comment(
        &self,
        number: &str,
        req: &CreateCommentRequest,
    ) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/comments", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出 Issue 当前 assignee。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_issue_assignees(&self, number: &str) -> Result<Vec<IssueAssignee>, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 为 Issue 添加 assignee。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn add_issue_assignees(
        &self,
        number: &str,
        req: &AddAssigneesRequest,
    ) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出 Issue 标签
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_issue_labels(&self, number: &str) -> Result<Vec<IssueLabel>, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 添加 Issue 标签
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn add_issue_labels(
        &self,
        number: &str,
        req: &IssueLabelRequest,
    ) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置（替换）Issue 标签
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn set_issue_labels(
        &self,
        number: &str,
        req: &IssueLabelRequest,
    ) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除 Issue 指定标签
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn remove_issue_label(&self, number: &str, label_name: &str) -> Result<(), ApiError> {
        let number = encode(number);
        let label_name = encode(label_name);
        let url = format!(
            "{}{}/-/issues/{number}/labels/{label_name}",
            self.base_url, self.repo
        );
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 清空 Issue 所有标签
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn clear_issue_labels(&self, number: &str) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }
}
