use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;
use urlencoding::encode;

impl CnbClient {
    pub async fn list_issues(&self, opts: &ListIssuesOptions) -> Result<Vec<Issue>, ApiError> {
        let mut url = format!("{}{}/-/issues?page={}&page_size={}&state={}",
            self.base_url, self.repo, opts.page, opts.page_size, encode(&opts.state));
        if let Some(ref keyword) = opts.keyword {
            url.push_str(&format!("&keyword={}", encode(keyword)));
        }
        if let Some(ref priority) = opts.priority {
            url.push_str(&format!("&priority={}", encode(priority)));
        }
        if let Some(ref labels) = opts.labels {
            url.push_str(&format!("&labels={}", encode(labels)));
        }
        if let Some(ref labels_operator) = opts.labels_operator {
            url.push_str(&format!("&labels_operator={}", encode(labels_operator)));
        }
        if let Some(ref authors) = opts.authors {
            url.push_str(&format!("&authors={}", encode(authors)));
        }
        if let Some(ref assignees) = opts.assignees {
            url.push_str(&format!("&assignees={}", encode(assignees)));
        }
        if let Some(ref begin) = opts.updated_time_begin {
            url.push_str(&format!("&updated_time_begin={}", encode(begin)));
        }
        if let Some(ref end) = opts.updated_time_end {
            url.push_str(&format!("&updated_time_end={}", encode(end)));
        }
        if let Some(ref begin) = opts.close_time_begin {
            url.push_str(&format!("&close_time_begin={}", encode(begin)));
        }
        if let Some(ref end) = opts.close_time_end {
            url.push_str(&format!("&close_time_end={}", encode(end)));
        }
        if let Some(ref order_by) = opts.order_by {
            url.push_str(&format!("&order_by={}", encode(order_by)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn list_all_issues(&self, state: &str) -> Result<Vec<Issue>, ApiError> {
        let page_size = 100u32;
        let mut all = Vec::new();
        let mut page = 1u32;
        loop {
            let opts = ListIssuesOptions {
                state: state.to_string(),
                page,
                page_size,
                ..Default::default()
            };
            let items = self.list_issues(&opts).await?;
            let count = items.len();
            all.extend(items);
            if (count as u32) < page_size { break; }
            page += 1;
        }
        Ok(all)
    }

    pub async fn get_issue(&self, number: &str) -> Result<IssueDetail, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn create_issue(&self, req: &CreateIssueRequest) -> Result<IssueDetail, ApiError> {
        let url = format!("{}{}/-/issues", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn update_issue(&self, number: &str, req: &UpdateIssueRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn list_issue_comments(&self, number: &str, page: u32, page_size: u32) -> Result<Vec<IssueComment>, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/comments?page={page}&page_size={page_size}",
            self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取全部 Issue 评论（自动分页）
    pub async fn list_all_issue_comments(&self, number: &str) -> Result<Vec<IssueComment>, ApiError> {
        self.paginate(|page, page_size| self.list_issue_comments(number, page, page_size)).await
    }

    pub async fn create_issue_comment(&self, number: &str, req: &CreateCommentRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/comments", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn list_issue_assignees(&self, number: &str) -> Result<Vec<IssueAssignee>, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn add_issue_assignees(&self, number: &str, req: &AddAssigneesRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出 Issue 标签
    pub async fn list_issue_labels(&self, number: &str) -> Result<Vec<IssueLabel>, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 添加 Issue 标签
    pub async fn add_issue_labels(&self, number: &str, req: &IssueLabelRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置（替换）Issue 标签
    pub async fn set_issue_labels(&self, number: &str, req: &IssueLabelRequest) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除 Issue 指定标签
    pub async fn remove_issue_label(&self, number: &str, label_name: &str) -> Result<(), ApiError> {
        let number = encode(number);
        let label_name = encode(label_name);
        let url = format!("{}{}/-/issues/{number}/labels/{label_name}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 清空 Issue 所有标签
    pub async fn clear_issue_labels(&self, number: &str) -> Result<(), ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/issues/{number}/labels", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }
}
