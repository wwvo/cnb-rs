//! 组织（Group）相关类型定义

use serde::{Deserialize, Serialize};

/// 上传组织 Logo 请求
#[derive(Debug, Serialize)]
pub struct UploadLogoRequest {
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    pub size: i64,
}

/// 上传组织 Logo 响应
#[derive(Debug, Deserialize)]
pub struct UploadLogoResponse {
    /// COS 上传 URL
    pub upload_url: String,
    /// 表单字段
    #[serde(default)]
    pub form: std::collections::HashMap<String, String>,
}

// ==================== 组织 CRUD ====================

/// 组织信息（列表和详情通用）
#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub remark: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub site: String,
    #[serde(default)]
    pub wechat_mp: String,
    #[serde(default)]
    pub readme_repo_path: String,
    #[serde(default)]
    pub member_count: i64,
    #[serde(default)]
    pub all_member_count: i64,
    #[serde(default)]
    pub sub_group_count: i64,
    #[serde(default)]
    pub all_sub_group_count: i64,
    #[serde(default)]
    pub sub_repo_count: i64,
    #[serde(default)]
    pub all_sub_repo_count: i64,
    #[serde(default)]
    pub follow_count: i64,
    #[serde(default)]
    pub freeze: bool,
    #[serde(default)]
    pub has_sub_group: bool,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// 创建组织请求
#[derive(Debug, Serialize)]
pub struct CreateGroupRequest {
    /// 组织路径（唯一标识）
    pub path: String,
    /// 组织描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 根组织绑定的域名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_domain: Option<String>,
}

/// 更新组织请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_mp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_repo_path: Option<String>,
}

/// 转移组织请求
#[derive(Debug, Serialize)]
pub struct TransferGroupRequest {
    pub source: String,
    pub target: String,
}

/// 列出顶层组织的查询参数
#[derive(Debug, Default)]
pub struct ListGroupsOptions {
    pub page: i32,
    pub page_size: i32,
    pub search: Option<String>,
    pub role: Option<String>,
}

impl ListGroupsOptions {
    #[must_use]
    pub fn query_string(&self) -> String {
        let mut params = vec![
            format!("page={}", self.page),
            format!("page_size={}", self.page_size),
        ];
        if let Some(ref s) = self.search {
            params.push(format!("search={}", urlencoding::encode(s)));
        }
        if let Some(ref r) = self.role {
            params.push(format!("role={}", urlencoding::encode(r)));
        }
        params.join("&")
    }
}

/// 列出子组织的查询参数
#[derive(Debug, Default)]
pub struct ListSubGroupsOptions {
    pub page: i32,
    pub page_size: i32,
    pub search: Option<String>,
}

impl ListSubGroupsOptions {
    #[must_use]
    pub fn query_string(&self) -> String {
        let mut params = vec![
            format!("page={}", self.page),
            format!("page_size={}", self.page_size),
        ];
        if let Some(ref s) = self.search {
            params.push(format!("search={}", urlencoding::encode(s)));
        }
        params.join("&")
    }
}

// ==================== 组织配置 ====================

/// 组织配置信息
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupSetting {
    #[serde(default)]
    pub hide_members: i32,
    #[serde(default)]
    pub hide_sub_groups: i32,
    #[serde(default)]
    pub group_protection: i32,
    #[serde(default)]
    pub show_private_repo_watermark: i32,
    #[serde(default)]
    pub email_verification: Vec<String>,
    #[serde(default)]
    pub can_show_members: bool,
    #[serde(default)]
    pub can_show_sub_groups: bool,
    #[serde(default)]
    pub can_show_watermark: bool,
}

/// 更新组织配置请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateGroupSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_members: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_sub_groups: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_protection: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_private_repo_watermark: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
}

// ==================== 特权额度 ====================

/// 组织特权额度
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialAmount {
    #[serde(default)]
    pub compute_build_corehour: f64,
    #[serde(default)]
    pub compute_build_desc: String,
    #[serde(default)]
    pub compute_build_expire: Option<String>,
    #[serde(default)]
    pub compute_build_gpu_corehour: f64,
    #[serde(default)]
    pub compute_build_gpu_desc: String,
    #[serde(default)]
    pub compute_build_gpu_expire: Option<String>,
    #[serde(default)]
    pub compute_develop_corehour: f64,
    #[serde(default)]
    pub compute_develop_desc: String,
    #[serde(default)]
    pub compute_develop_expire: Option<String>,
    #[serde(default)]
    pub compute_develop_gpu_corehour: f64,
    #[serde(default)]
    pub compute_develop_gpu_desc: String,
    #[serde(default)]
    pub compute_develop_gpu_expire: Option<String>,
    #[serde(default)]
    pub storage_git_gib: f64,
    #[serde(default)]
    pub storage_git_desc: String,
    #[serde(default)]
    pub storage_git_expire: Option<String>,
    #[serde(default)]
    pub storage_object_gib: f64,
    #[serde(default)]
    pub storage_object_desc: String,
    #[serde(default)]
    pub storage_object_expire: Option<String>,
}

// ==================== 成员管理 ====================

/// 组织成员信息
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupMember {
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
    pub inviter: Option<GroupMemberUser>,
}

/// 成员用户简要信息
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupMemberUser {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
}

/// 添加/更新成员请求
#[derive(Debug, Serialize)]
pub struct GroupMemberRequest {
    pub access_level: String,
    #[serde(default)]
    pub is_outside_collaborator: bool,
}

/// 列出成员查询参数
#[derive(Debug, Default)]
pub struct ListGroupMembersOptions {
    pub page: i32,
    pub page_size: i32,
    pub role: Option<String>,
    pub search: Option<String>,
}

impl ListGroupMembersOptions {
    #[must_use]
    pub fn query_string(&self) -> String {
        let mut params = vec![
            format!("page={}", self.page),
            format!("page_size={}", self.page_size),
        ];
        if let Some(ref r) = self.role {
            params.push(format!("role={}", urlencoding::encode(r)));
        }
        if let Some(ref s) = self.search {
            params.push(format!("search={}", urlencoding::encode(s)));
        }
        params.join("&")
    }
}

/// 成员权限信息
#[derive(Debug, Deserialize, Serialize)]
pub struct MemberAccessLevel {
    #[serde(default)]
    pub access_level: serde_json::Value,
    #[serde(default)]
    pub inherit: bool,
    #[serde(default)]
    pub read_privilege: bool,
    #[serde(default)]
    pub write_privilege: bool,
}

/// 成员权限层级信息
#[derive(Debug, Deserialize, Serialize)]
pub struct MemberAccessLevelInPath {
    #[serde(default)]
    pub access_level: serde_json::Value,
    #[serde(default)]
    pub path: String,
}

/// 继承成员分组
#[derive(Debug, Deserialize, Serialize)]
pub struct InheritMembersGroup {
    #[serde(default)]
    pub inherit_path: String,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub users: Vec<GroupMember>,
}
