//! cnb release asset-upload 子命令 - 上传附件到 Release

use anyhow::Result;
use clap::Parser;
use cnb_api::types::PostReleaseAssetUploadURLRequest;
use cnb_core::context::AppContext;

/// 上传附件到 Release
#[derive(Debug, Parser)]
pub struct AssetUploadArgs {
    /// Release 的 Tag 名称
    #[arg(short = 't', long = "tag-name")]
    pub tag_name: String,

    /// 要上传的文件路径
    #[arg(short = 'f', long = "file")]
    pub file_path: String,
}

/// 执行 release asset-upload 命令
pub async fn run(ctx: &AppContext, args: &AssetUploadArgs) -> Result<()> {
    let client = ctx.api_client()?;

    // 检查文件是否存在
    let path = std::path::Path::new(&args.file_path);
    if !path.is_file() {
        anyhow::bail!("{} 不是有效的文件", args.file_path);
    }

    let metadata = std::fs::metadata(path)?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");

    // 根据 tag 获取 release
    let release = client
        .get_release_by_tag(client.repo(), &args.tag_name)
        .await?;

    // 获取上传 URL
    let req = PostReleaseAssetUploadURLRequest {
        asset_name: file_name.to_string(),
        size: metadata.len() as i64,
    };
    let upload_info = client
        .get_release_asset_upload_url(client.repo(), &release.id, &req)
        .await?;

    // 读取文件并上传到 COS
    let file_data = std::fs::read(path)?;
    let http = reqwest::Client::new();
    let resp: reqwest::Response = http
        .put(&upload_info.upload_url)
        .header("Content-Type", "application/octet-stream")
        .body(file_data)
        .send()
        .await?;

    let status = resp.status().as_u16();
    if status < 200 || status >= 300 {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("上传失败 (HTTP {status}): {body}");
    }

    // 确认上传
    let confirm_resp: reqwest::Response = http
        .post(&upload_info.verify_url)
        .header("Accept", "application/vnd.cnb.api+json")
        .header("Authorization", format!("Bearer {}", client.token()))
        .send()
        .await?;

    let confirm_status = confirm_resp.status().as_u16();
    if confirm_status != 200 {
        let body = confirm_resp.text().await.unwrap_or_default();
        anyhow::bail!("上传确认失败 (HTTP {confirm_status}): {body}");
    }

    println!("文件 {} 已上传到 Release {}", file_name, args.tag_name);

    Ok(())
}
