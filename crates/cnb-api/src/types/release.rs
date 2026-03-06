//! Release 相关类型

use serde::{Deserialize, Serialize};

/// Release 信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub id: String,
    pub name: String,
    pub tag_name: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub is_latest: bool,
    #[serde(default)]
    pub published_at: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

/// Release 附件
#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseAsset {
    pub id: String,
    pub name: String,
    pub size: i64,
    #[serde(default)]
    pub created_at: String,
}

/// 创建 Release 请求
#[derive(Debug, Serialize)]
pub struct CreateReleaseRequest {
    pub tag_name: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub make_latest: String,
    pub target_commitish: String,
}

/// Release 附件上传 URL 响应
#[derive(Debug, Deserialize)]
pub struct ReleaseAssetUploadURL {
    pub upload_url: String,
    pub verify_url: String,
}

/// 请求附件上传 URL 的请求体
#[derive(Debug, Serialize)]
pub struct PostReleaseAssetUploadURLRequest {
    pub asset_name: String,
    pub size: i64,
}
