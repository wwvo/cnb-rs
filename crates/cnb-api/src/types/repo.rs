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

/// Fork 列表响应
#[derive(Debug, Deserialize, Serialize)]
pub struct ForkList {
    #[serde(default)]
    pub fork_tree_count: u64,
    #[serde(default)]
    pub forks: Vec<ForkInfo>,
}

/// Fork 信息
#[derive(Debug, Deserialize, Serialize)]
pub struct ForkInfo {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub fork_count: u64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub freeze: bool,
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

/// 更新仓库请求体
#[derive(Debug, Serialize)]
pub struct UpdateRepoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

/// 贡献者趋势响应
#[derive(Debug, Deserialize, Serialize)]
pub struct ContributorTrend {
    #[serde(default)]
    pub user_total: u64,
    #[serde(default)]
    pub week_total: u64,
    #[serde(default)]
    pub users_data: Vec<ContributorData>,
    #[serde(default)]
    pub with_line_counts: bool,
}

/// 贡献者数据
#[derive(Debug, Deserialize, Serialize)]
pub struct ContributorData {
    #[serde(default)]
    pub author: ContributorAuthor,
    #[serde(default)]
    pub commit_count: u64,
    #[serde(default)]
    pub weeks: Vec<ContributorWeek>,
}

/// 贡献者作者信息
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContributorAuthor {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub user_name: String,
}

/// 贡献者周数据
#[derive(Debug, Deserialize, Serialize)]
pub struct ContributorWeek {
    #[serde(default)]
    pub a: u64,
    #[serde(default)]
    pub c: u64,
    #[serde(default)]
    pub d: u64,
    #[serde(default)]
    pub w: u64,
}

/// 安全概览响应
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityOverview {
    #[serde(default)]
    pub risk_cnt: Option<SecurityRiskCount>,
    #[serde(default)]
    pub code_sensitive: Option<SecurityModule>,
    #[serde(default)]
    pub code_vulnerability: Option<SecurityVulnerability>,
    #[serde(default)]
    pub code_issue: Option<SecurityModule>,
}

/// 安全风险统计
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityRiskCount {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub code_sensitive_enable: bool,
    #[serde(default)]
    pub code_sensitive_risk_cnt: u64,
    #[serde(default)]
    pub code_vulnerability_enable: bool,
    #[serde(default)]
    pub code_vulnerability_risk_cnt: u64,
    #[serde(default)]
    pub code_issue_enable: bool,
    #[serde(default)]
    pub code_issue_risk_cnt: u64,
}

/// 安全模块（敏感信息 / 代码问题）
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityModule {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub open: u64,
    #[serde(default)]
    pub ignored: u64,
    #[serde(default)]
    pub high_count: u64,
    #[serde(default)]
    pub medium_count: u64,
    #[serde(default)]
    pub low_count: u64,
    #[serde(default)]
    pub critical_count: u64,
}

/// 安全漏洞模块
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityVulnerability {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub open: u64,
    #[serde(default)]
    pub ignored: u64,
    #[serde(default)]
    pub critical_vul_open_cnt: u64,
    #[serde(default)]
    pub high_vul_open_cnt: u64,
    #[serde(default)]
    pub medium_vul_open_cnt: u64,
    #[serde(default)]
    pub low_vul_open_cnt: u64,
}

/// 活跃用户信息
#[derive(Debug, Deserialize, Serialize)]
pub struct TopContributor {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub repo_count: u64,
    #[serde(default)]
    pub stars_count: u64,
}

/// 仓库资产记录
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetRecord {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub origin_path: String,
    #[serde(default)]
    pub record_type: Option<serde_json::Value>,
    #[serde(default)]
    pub referer: String,
    #[serde(default)]
    pub size_in_byte: u64,
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
            params.push(format!("search={}", urlencoding::encode(s)));
        }
        if let Some(f) = &self.filter_type {
            params.push(format!("filter_type={}", urlencoding::encode(f)));
        }
        if let Some(o) = &self.order_by {
            params.push(format!("order_by={}", urlencoding::encode(o)));
        }
        if self.desc {
            params.push("desc=true".to_string());
        }
        params.join("&")
    }
}

// ============================
// 仓库设置相关类型
// ============================

/// 分支保护规则
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct BranchProtection {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub rule: String,
    #[serde(default)]
    pub allow_pushes: bool,
    #[serde(default)]
    pub allow_master_pushes: bool,
    #[serde(default)]
    pub allow_force_pushes: bool,
    #[serde(default)]
    pub allow_master_force_pushes: bool,
    #[serde(default)]
    pub allow_creation: bool,
    #[serde(default)]
    pub allow_master_creation: bool,
    #[serde(default)]
    pub allow_deletions: bool,
    #[serde(default)]
    pub allow_master_deletions: bool,
    #[serde(default)]
    pub allow_master_manual_merge: bool,
    #[serde(default)]
    pub required_must_auto_merge: bool,
    #[serde(default)]
    pub required_must_push_via_pull_request: bool,
    #[serde(default)]
    pub required_pull_request_reviews: bool,
    #[serde(default)]
    pub required_approved_review_count: u8,
    #[serde(default)]
    pub required_approved_review_ratio: u8,
    #[serde(default)]
    pub required_master_approve: bool,
    #[serde(default)]
    pub required_linear_history: bool,
    #[serde(default)]
    pub required_status_checks: bool,
}

/// 创建/更新分支保护规则请求体
#[derive(Debug, Default, Serialize)]
pub struct BranchProtectionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_pushes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_master_pushes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_master_force_pushes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_creation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_master_creation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_master_deletions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_master_manual_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_must_auto_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_must_push_via_pull_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_approved_review_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_approved_review_ratio: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_master_approve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<bool>,
}

/// 合并请求设置
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequestSettings {
    #[serde(default)]
    pub allow_merge_commit_merge: bool,
    #[serde(default)]
    pub allow_rebase_merge: bool,
    #[serde(default)]
    pub allow_squash_merge: bool,
    #[serde(default)]
    pub master_auto_as_reviewer: bool,
    #[serde(default)]
    pub merge_commit_message_style: String,
    #[serde(default)]
    pub squash_commit_message_style: String,
}

/// 更新合并请求设置请求体
#[derive(Debug, Default, Serialize)]
pub struct PullRequestSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_auto_as_reviewer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_message_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash_commit_message_style: Option<String>,
}

/// 推送限制设置
#[derive(Debug, Deserialize, Serialize)]
pub struct PushLimitSettings {
    #[serde(default)]
    pub check_single_push_number: bool,
    #[serde(default)]
    pub allow_single_push_number: u32,
    #[serde(default)]
    pub only_master_can_push_tag: bool,
    #[serde(default)]
    pub push_commit_must_be: String,
}

/// 更新推送限制设置请求体
#[derive(Debug, Default, Serialize)]
pub struct PushLimitSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_single_push_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_push_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_master_can_push_tag: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_commit_must_be: Option<String>,
}

/// 流水线构建设置
#[derive(Debug, Deserialize, Serialize)]
pub struct PipelineSettings {
    #[serde(default)]
    pub auto_trigger: bool,
    #[serde(default)]
    pub forked_repo_auto_trigger: bool,
}

/// 更新流水线构建设置请求体
#[derive(Debug, Default, Serialize)]
pub struct PipelineSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_trigger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forked_repo_auto_trigger: Option<bool>,
}
