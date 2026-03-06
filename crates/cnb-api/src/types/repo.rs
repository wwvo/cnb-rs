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

/// 创建仓库请求体
#[derive(Debug, Serialize)]
pub struct CreateRepoRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// 仓库列表查询选项
#[derive(Debug)]
pub struct ListReposOptions {
    pub page: u32,
    pub page_size: u32,
    pub search: Option<String>,
    pub filter_type: Option<String>,
    pub order_by: Option<String>,
    pub desc: bool,
}

impl Default for ListReposOptions {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 30,
            search: None,
            filter_type: None,
            order_by: None,
            desc: false,
        }
    }
}

impl ListReposOptions {
    /// 拼接通用查询参数
    pub fn query_string(&self) -> String {
        let mut params = vec![
            format!("page={}", self.page),
            format!("page_size={}", self.page_size),
        ];
        if let Some(s) = &self.search {
            params.push(format!("search={s}"));
        }
        if let Some(f) = &self.filter_type {
            params.push(format!("filter_type={f}"));
        }
        if let Some(o) = &self.order_by {
            params.push(format!("order_by={o}"));
        }
        if self.desc {
            params.push("desc=true".to_string());
        }
        params.join("&")
    }
}
