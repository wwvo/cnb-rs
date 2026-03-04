//! Star 相关类型定义

use serde::Deserialize;

/// Star 用户列表响应
#[derive(Debug, Deserialize)]
pub struct StarUsers {
    /// Star 总数
    pub total: i64,
    /// Star 用户列表
    #[serde(default)]
    pub users: Vec<StarUser>,
}

/// Star 用户信息
#[derive(Debug, Deserialize)]
pub struct StarUser {
    /// 用户名
    #[serde(default)]
    pub username: String,
    /// Star 时间
    #[serde(default)]
    pub stared_at: String,
}
