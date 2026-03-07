//! 标签相关类型

use serde::{Deserialize, Serialize};

/// 仓库标签
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Label {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub description: String,
}

/// 创建标签请求
#[derive(Debug, Serialize)]
pub struct CreateLabelRequest {
    pub name: String,
    pub color: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// 更新标签请求
#[derive(Debug, Serialize)]
pub struct UpdateLabelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 标签列表请求（添加/设置标签）
#[derive(Debug, Serialize)]
pub struct LabelListRequest {
    pub labels: Vec<String>,
}
