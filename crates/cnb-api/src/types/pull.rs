//! Pull Request 相关类型

use serde::{Deserialize, Serialize};

/// Pull Request 基础信息（列表接口返回）
#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub number: String,
    pub title: String,
    pub state: String,
    #[serde(default)]
    pub blocked_on: String,
}

/// Pull Request 详情（创建/更新接口返回）
#[derive(Debug, Deserialize)]
pub struct Pull {
    pub number: String,
    pub title: String,
    pub body: String,
    pub state: String,
}

/// 合并 Pull Request 响应
#[derive(Debug, Deserialize)]
pub struct MergePullResponse {
    pub merged: bool,
    pub sha: String,
    pub message: String,
}

/// 创建 Pull Request 请求
#[derive(Debug, Serialize)]
pub struct CreatePullRequest {
    pub base: String,
    pub head: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub head_repo: String,
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
}

/// 更新 Pull Request 请求
#[derive(Debug, Serialize)]
pub struct UpdatePullRequest {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
}

/// 合并 Pull Request 请求
#[derive(Debug, Serialize)]
pub struct MergePullRequestBody {
    pub commit_title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub commit_message: String,
    pub merge_style: String,
}

/// Pull Request 列表查询参数
#[derive(Debug, Default)]
pub struct ListPullsOptions {
    pub state: String,
    pub page: u32,
    pub page_size: u32,
    pub reviewers: Option<String>,
    pub authors: Option<String>,
}

/// Head 引用信息（用于获取默认分支）
#[derive(Debug, Deserialize)]
pub struct HeadRef {
    pub name: String,
}
