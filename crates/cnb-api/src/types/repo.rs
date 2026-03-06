//! 仓库相关类型

use serde::{Deserialize, Serialize};

/// 仓库信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Repo {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub star_count: u64,
    #[serde(default)]
    pub fork_count: u64,
    #[serde(default)]
    pub open_issue_count: u64,
    #[serde(default)]
    pub open_pull_request_count: u64,
    #[serde(default)]
    pub visibility_level: Option<serde_json::Value>,
    #[serde(default)]
    pub languages: Option<RepoLanguage>,
    #[serde(default)]
    pub last_updated_at: Option<RepoTime>,
    #[serde(default)]
    pub web_url: String,
    #[serde(default)]
    pub stared: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub site: String,
    #[serde(default)]
    pub topics: String,
}

/// 仓库语言信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RepoLanguage {
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub language: String,
}

/// 仓库时间信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RepoTime {
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub valid: bool,
}
