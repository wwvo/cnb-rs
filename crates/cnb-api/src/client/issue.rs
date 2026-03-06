use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn list_issues(&self, opts: &ListIssuesOptions) -> Result<Vec<Issue>, ApiError> {
        let mut url = format!("{}{}/-/issues?page={}&page_size={}&state={}",
            self.base_url, self.repo, opts.page, opts.page_size, opts.state);
        if let Some(ref assignees) = opts.assignees {
            url.push_str(&format!("&assignees={assignees}"));
        }
        if let Some(ref authors) = opts.authors {
            url.push_str(&format!("&authors={authors}"));
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
        let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn list_issue_comments(&self, number: &str, page: u32, page_size: u32) -> Result<Vec<IssueComment>, ApiError> {
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
        let url = format!("{}{}/-/issues/{number}/comments", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn list_issue_assignees(&self, number: &str) -> Result<Vec<IssueAssignee>, ApiError> {
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn add_issue_assignees(&self, number: &str, req: &AddAssigneesRequest) -> Result<(), ApiError> {
        let url = format!("{}{}/-/issues/{number}/assignees", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }
}
