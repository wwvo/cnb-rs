//! cnb commit asset-upload 子命令 - 上传附件到 Commit

use anyhow::Result;
use clap::Parser;
use cnb_api::types::PostCommitAssetUploadURLRequest;
use cnb_core::context::AppContext;
use cnb_core::upload;
use cnb_tui::success;

/// 上传附件到 Commit
#[derive(Debug, Parser)]
pub struct AssetUploadArgs {
    /// Commit 的 SHA1
    #[arg(long = "sha1")]
    pub sha1: String,

    /// 要上传的文件路径
    #[arg(short = 'f', long = "file")]
    pub file_path: String,
}

/// 执行 commit asset-upload 命令
pub async fn run(ctx: &AppContext, args: &AssetUploadArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let (file_name, file_size) = upload::validate_file(&args.file_path)?;

    let req = PostCommitAssetUploadURLRequest {
        asset_name: file_name.to_string(),
        size: file_size,
    };
    let upload_info = client.get_commit_asset_upload_url(&args.sha1, &req).await?;

    let path = std::path::Path::new(&args.file_path);
    upload::upload_and_confirm(
        client.http_client(),
        path,
        &upload_info.upload_url,
        &upload_info.verify_url,
        client.token(),
    )
    .await?;

    success!("文件 {file_name} 已上传到 Commit {}", args.sha1);

    Ok(())
}
