//! cnb release asset-upload 子命令 - 上传附件到 Release

use anyhow::Result;
use clap::Parser;
use cnb_api::types::PostReleaseAssetUploadURLRequest;
use cnb_core::context::AppContext;
use cnb_core::upload;
use cnb_tui::success;

/// 上传附件到 Release
#[derive(Debug, Parser)]
pub struct AssetUploadArgs {
    /// Release 的 Tag 名称
    #[arg(short = 't', long = "tag", alias = "tag-name")]
    pub tag_name: String,

    /// 要上传的文件路径
    #[arg(short = 'f', long = "file")]
    pub file_path: String,
}

/// 执行 release asset-upload 命令
pub async fn run(ctx: &AppContext, args: &AssetUploadArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let (file_name, file_size) = upload::validate_file(&args.file_path)?;

    let release = client
        .get_release_by_tag(client.repo(), &args.tag_name)
        .await?;

    let req = PostReleaseAssetUploadURLRequest {
        asset_name: file_name.to_string(),
        size: file_size,
    };
    let upload_info = client
        .get_release_asset_upload_url(client.repo(), &release.id, &req)
        .await?;

    let path = std::path::Path::new(&args.file_path);
    upload::upload_and_confirm(
        client.http_client(),
        path,
        &upload_info.upload_url,
        &upload_info.verify_url,
        client.token(),
    )
    .await?;

    success!("文件 {file_name} 已上传到 Release {}", args.tag_name);

    Ok(())
}
