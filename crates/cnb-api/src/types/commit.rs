//! Commit 相关类型定义

use serde::{Deserialize, Serialize};

/// Commit 信息
#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetail,
}

/// Commit 详情（嵌套）
#[derive(Debug, Deserialize)]
pub struct CommitDetail {
    pub committer: CommitPerson,
}

/// Commit 提交者/作者信息
#[derive(Debug, Deserialize)]
pub struct CommitPerson {
    pub date: String,
}

/// Commit 附件
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitAsset {
    pub id: String,
    pub name: String,
    pub size_in_byte: i64,
    pub created_at: String,
}

/// 获取 Commit 附件上传 URL 的请求
#[derive(Debug, Serialize)]
pub struct PostCommitAssetUploadURLRequest {
    pub asset_name: String,
    pub size: i64,
}

/// Commit 附件上传 URL 响应
#[derive(Debug, Deserialize)]
pub struct CommitAssetUploadURL {
    pub upload_url: String,
    pub verify_url: String,
}
