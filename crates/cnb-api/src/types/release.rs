//! Release 相关类型

use serde::{Deserialize, Serialize};

/// Release 作者
#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseAuthor {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
}

/// Release 信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub id: String,
    pub name: String,
    pub tag_name: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub is_latest: bool,
    #[serde(default)]
    pub published_at: String,
    #[serde(default)]
    pub tag_commitish: String,
    #[serde(default)]
    pub author: Option<ReleaseAuthor>,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Release 附件上传者
#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseAssetUploader {
    #[serde(default)]
    pub username: String,
}

/// Release 附件
#[derive(Debug, Deserialize, Serialize)]
pub struct ReleaseAsset {
    pub id: String,
    pub name: String,
    pub size: i64,
    #[serde(default)]
    pub download_count: i64,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub brower_download_url: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub uploader: Option<ReleaseAssetUploader>,
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

/// 更新 Release 请求
#[derive(Debug, Serialize)]
pub struct UpdateReleaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make_latest: Option<String>,
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
