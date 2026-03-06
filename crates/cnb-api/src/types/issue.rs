//! Issue 相关类型

use serde::{Deserialize, Serialize};

/// Issue 基础信息（列表接口返回）
#[derive(Debug, Deserialize)]
pub struct Issue {
    pub number: String,
    pub title: String,
    pub state: String,
    #[serde(default)]
    pub last_acted_at: String,
}

/// Issue 详细信息（单个 Issue 接口返回）
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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
