//! 知识库相关类型定义

use serde::{Deserialize, Serialize};

/// 知识库 Embedding 模型
#[derive(Debug, Deserialize)]
pub struct KnowledgeModel {
    /// 模型名称
    pub name: String,
}

/// 知识库信息
#[derive(Debug, Deserialize)]
pub struct KnowledgeBaseInfo {
    /// 知识库 ID
    #[serde(default)]
    pub id: String,
    /// 最后提交 SHA
    #[serde(default)]
    pub last_commit_sha: String,
    /// Embedding 模型
    #[serde(default)]
    pub embedding_model: KnowledgeEmbeddingModel,
    /// 包含规则
    #[serde(default)]
    pub include: String,
    /// 排除规则
    #[serde(default)]
    pub exclude: String,
}

/// Embedding 模型信息
#[derive(Debug, Default, Deserialize)]
pub struct KnowledgeEmbeddingModel {
    /// 模型名称
    #[serde(default)]
    pub name: String,
}

/// 知识库查询请求
#[derive(Debug, Serialize)]
pub struct QueryKnowledgeBaseRequest {
    /// 查询文本
    pub query: String,
    /// 分数阈值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
    /// 返回结果数量上限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
}

/// 知识库查询结果条目
#[derive(Debug, Deserialize)]
pub struct KnowledgeQueryResult {
    /// 分数
    #[serde(default)]
    pub score: f64,
    /// 元数据
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}
