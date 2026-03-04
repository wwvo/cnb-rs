//! 云原生工作区相关类型定义

use serde::Deserialize;

/// 工作区列表响应
#[derive(Debug, Deserialize)]
pub struct WorkspaceListResponse {
    /// 总数
    #[serde(default)]
    pub total: i64,
    /// 工作区列表
    #[serde(default)]
    pub list: Vec<WorkspaceInfo>,
}

/// 工作区信息
#[derive(Debug, Deserialize)]
pub struct WorkspaceInfo {
    /// 仓库路径
    #[serde(default)]
    pub slug: String,
    /// Pipeline ID
    #[serde(default)]
    pub pipeline_id: String,
    /// 状态（running / closed）
    #[serde(default)]
    pub status: String,
    /// 分支
    #[serde(default)]
    pub branch: String,
}
