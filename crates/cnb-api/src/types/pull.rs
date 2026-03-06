//! Pull Request 相关类型

use serde::{Deserialize, Serialize};

/// Pull Request 基础信息（列表接口返回）
#[derive(Debug, Deserialize, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// 合并方式
#[derive(Debug, Clone, clap::ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeStyle {
    Merge,
    Squash,
    Rebase,
}

/// 合并 Pull Request 请求
#[derive(Debug, Serialize)]
pub struct MergePullRequestBody {
    pub commit_title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub commit_message: String,
    pub merge_style: MergeStyle,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_request_deserialize() {
        let json = r#"{"number": "10", "title": "feat: add feature", "state": "open", "blocked_on": ""}"#;
        let pr: PullRequest = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert_eq!(pr.number, "10");
        assert_eq!(pr.title, "feat: add feature");
        assert_eq!(pr.state, "open");
    }

    #[test]
    fn pull_request_deserialize_defaults() {
        let json = r#"{"number": "1", "title": "test", "state": "open"}"#;
        let pr: PullRequest = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert_eq!(pr.blocked_on, "");
    }

    #[test]
    fn merge_style_serialize() {
        let merge = serde_json::to_string(&MergeStyle::Merge)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert_eq!(merge, r#""merge""#);

        let squash = serde_json::to_string(&MergeStyle::Squash)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert_eq!(squash, r#""squash""#);

        let rebase = serde_json::to_string(&MergeStyle::Rebase)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert_eq!(rebase, r#""rebase""#);
    }

    #[test]
    fn create_pull_request_serialize_skip_empty() {
        let req = CreatePullRequest {
            base: "main".to_string(),
            head: "feature".to_string(),
            head_repo: String::new(),
            title: "test PR".to_string(),
            body: String::new(),
        };
        let json = serde_json::to_string(&req)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert!(!json.contains("head_repo"));
        assert!(!json.contains("body"));
        assert!(json.contains("base"));
        assert!(json.contains("head"));
    }

    #[test]
    fn update_pull_request_serialize_skip_none() {
        let req = UpdatePullRequest {
            title: None,
            body: None,
            state: None,
        };
        let json = serde_json::to_string(&req)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert_eq!(json, "{}");
    }

    #[test]
    fn merge_pull_request_body_serialize() {
        let body = MergePullRequestBody {
            commit_title: "merge: feature".to_string(),
            commit_message: String::new(),
            merge_style: MergeStyle::Squash,
        };
        let json = serde_json::to_string(&body)
            .unwrap_or_else(|e| panic!("序列化失败: {e}"));
        assert!(json.contains(r#""merge_style":"squash""#));
        assert!(!json.contains("commit_message"));
    }

    #[test]
    fn merge_pull_response_deserialize() {
        let json = r#"{"merged": true, "sha": "abc123", "message": "ok"}"#;
        let resp: MergePullResponse = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("反序列化失败: {e}"));
        assert!(resp.merged);
        assert_eq!(resp.sha, "abc123");
    }
}
