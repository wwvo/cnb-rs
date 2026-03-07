//! 制品库管理相关类型

use serde::{Deserialize, Serialize};

/// 制品库信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Registry {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub visibility_level: serde_json::Value,
    #[serde(default)]
    pub artifact_policy: String,
    #[serde(default)]
    pub overwrite_policy: String,
    #[serde(default)]
    pub pkg_count: u64,
    #[serde(default)]
    pub used_size: u64,
    #[serde(default)]
    pub freeze: bool,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub stared: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub last_push_time: String,
    #[serde(default)]
    pub last_push_user: serde_json::Value,
}

/// 制品信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Package {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub package: String,
    #[serde(default)]
    pub package_type: serde_json::Value,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub is_dir: bool,
    #[serde(default)]
    pub pull_count: u64,
    #[serde(default)]
    pub recent_pull_count: u64,
    #[serde(default)]
    pub last_artifact_name: String,
    #[serde(default)]
    pub last_pusher: serde_json::Value,
}

/// 制品详情（通用结构，按类型返回不同内容）
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PackageDetail {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// 制品标签（通用结构）
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PackageTag {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// 设置制品库可见性请求
#[derive(Debug, Serialize)]
pub struct SetRegistryVisibilityRequest {
    pub visibility: String,
}
