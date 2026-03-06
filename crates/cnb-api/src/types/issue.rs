//! Issue 相关类型

use serde::{Deserialize, Serialize};

/// Issue 基础信息（列表接口返回）
#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: String,
    pub title: String,
    pub state: String,
    #[serde(default)]
    pub last_acted_at: String,
}

/// Issue 详细信息（单个 Issue 接口返回）
#[derive(Debug, Deserialize, Serialize)]
pub struct IssueDetail {
    pub number: String,
    pub title: String,
    pub body: String,
    pub state: String,
}

/// 创建 Issue 请求
#[derive(Debug, Serialize)]
pub struct CreateIssueRequest {
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub priority: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// 更新 Issue 请求
#[derive(Debug, Serialize)]
pub struct UpdateIssueRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

/// Issue 评论
#[derive(Debug, Deserialize)]
pub struct IssueComment {
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub author: CommentAuthor,
}

/// 评论作者
#[derive(Debug, Deserialize)]
pub struct CommentAuthor {
    pub username: String,
    pub nickname: String,
}

/// 创建评论请求
#[derive(Debug, Serialize)]
pub struct CreateCommentRequest {
    pub body: String,
}

/// Issue 处理人
#[derive(Debug, Deserialize, Serialize)]
pub struct IssueAssignee {
    pub username: String,
}

/// 添加处理人请求
#[derive(Debug, Serialize)]
pub struct AddAssigneesRequest {
    pub assignees: Vec<String>,
}

/// Issue 列表查询参数
#[derive(Debug, Default)]
pub struct ListIssuesOptions {
    pub state: String,
    pub page: u32,
    pub page_size: u32,
    pub assignees: Option<String>,
    pub authors: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_deserialize() {
        let json = r#"{"number": "42", "title": "Bug report", "state": "open", "last_acted_at": "2025-01-01T00:00:00Z"}"#;
        let issue: Issue = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert_eq!(issue.number, "42");
        assert_eq!(issue.title, "Bug report");
        assert_eq!(issue.state, "open");
        assert_eq!(issue.last_acted_at, "2025-01-01T00:00:00Z");
    }

    #[test]
    fn issue_deserialize_defaults() {
        let json = r#"{"number": "1", "title": "test", "state": "closed"}"#;
        let issue: Issue = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert_eq!(issue.last_acted_at, "");
    }

    #[test]
    fn issue_detail_deserialize() {
        let json = r#"{"number": "1", "title": "test", "body": "description", "state": "open"}"#;
        let detail: IssueDetail = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert_eq!(detail.body, "description");
    }

    #[test]
    fn create_issue_request_serialize_minimal() {
        let req = CreateIssueRequest {
            title: "test".to_string(),
            body: String::new(),
            priority: String::new(),
            labels: vec![],
            assignees: vec![],
            start_date: None,
            end_date: None,
        };
        let json = serde_json::to_string(&req)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        // 空字段应被 skip
        assert!(!json.contains("body"));
        assert!(!json.contains("priority"));
        assert!(!json.contains("labels"));
        assert!(!json.contains("assignees"));
        assert!(json.contains("title"));
    }

    #[test]
    fn create_issue_request_serialize_full() {
        let req = CreateIssueRequest {
            title: "test".to_string(),
            body: "desc".to_string(),
            priority: "p0".to_string(),
            labels: vec!["bug".to_string()],
            assignees: vec!["user1".to_string()],
            start_date: Some("2025-01-01".to_string()),
            end_date: Some("2025-12-31".to_string()),
        };
        let json = serde_json::to_string(&req)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert!(json.contains("body"));
        assert!(json.contains("priority"));
        assert!(json.contains("labels"));
        assert!(json.contains("start_date"));
    }

    #[test]
    fn update_issue_request_serialize_skip_none() {
        let req = UpdateIssueRequest {
            state: None,
            state_reason: None,
        };
        let json = serde_json::to_string(&req)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert_eq!(json, "{}");
    }

    #[test]
    fn update_issue_request_serialize_with_values() {
        let req = UpdateIssueRequest {
            state: Some("closed".to_string()),
            state_reason: Some("completed".to_string()),
        };
        let json = serde_json::to_string(&req)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert!(json.contains(r#""state":"closed""#));
        assert!(json.contains(r#""state_reason":"completed""#));
    }

    #[test]
    fn issue_comment_deserialize() {
        let json = r#"{"body": "comment text", "created_at": "2025-01-01T00:00:00Z", "updated_at": "2025-01-02T00:00:00Z", "author": {"username": "user1", "nickname": "User One"}}"#;
        let comment: IssueComment = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert_eq!(comment.body, "comment text");
        assert_eq!(comment.author.username, "user1");
    }
}
