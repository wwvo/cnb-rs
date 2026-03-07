//! 成员管理相关类型

use serde::{Deserialize, Serialize};

/// 成员信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MemberInfo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub access_level: serde_json::Value,
    #[serde(default)]
    pub join_time: String,
    #[serde(default)]
    pub freeze: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub inviter: Option<MemberInviter>,
}

/// 邀请人信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MemberInviter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub avatar: String,
}

/// 外部贡献者
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OutsideCollaborator {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub access_level: serde_json::Value,
    #[serde(default)]
    pub join_time: String,
    #[serde(default)]
    pub freeze: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 添加/更新成员请求
#[derive(Debug, Serialize)]
pub struct MemberRequest {
    pub access_level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_outside_collaborator: Option<bool>,
}
