//! cnb group update-logo 子命令

use anyhow::{Result, bail};
use clap::Parser;
use cnb_api::types::UploadLogoRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;
use std::path::Path;

/// 更新组织 Logo 参数
#[derive(Debug, Parser)]
pub struct UpdateLogoArgs {
    /// 组织名称
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// Logo 文件路径
    #[arg(long = "logo-path")]
    pub logo_path: String,
}

/// 更新组织 Logo
pub async fn run(ctx: &AppContext, args: &UpdateLogoArgs) -> Result<()> {
    let path = Path::new(&args.logo_path);
    if !path.is_file() {
        bail!("{} 不是有效的文件", args.logo_path);
    }

    let metadata = std::fs::metadata(path)?;
    let file_name = path
        .file_name()
        .map_or_else(|| "logo".to_string(), |n| n.to_string_lossy().to_string());

    let client = ctx.api_client()?;

    // 获取上传信息
    let req = UploadLogoRequest {
        name: file_name,
        size: metadata.len() as i64,
    };
    let upload_info = client.upload_logo_info(&args.group, &req).await?;

    // 读取文件
    let data = std::fs::read(path)?;

    // 上传到 COS
    client
        .post_to_cos(&upload_info.upload_url, &upload_info.form, data)
        .await?;

    success!("组织 Logo 更新成功");

    Ok(())
}
