//! 通用文件上传工具
//!
//! COS 上传流程：PUT 文件到 upload_url → POST 确认到 verify_url

use std::path::Path;
use tokio_util::io::ReaderStream;

/// 上传文件到 COS 并确认
///
/// 1. 流式读取本地文件（避免大文件全量加载到内存）
/// 2. PUT 上传到 `upload_url`
/// 3. POST 确认到 `verify_url`（带 Bearer Token 认证）
pub async fn upload_and_confirm(
    http: &reqwest::Client,
    file_path: &Path,
    upload_url: &str,
    verify_url: &str,
    token: &str,
) -> anyhow::Result<()> {
    let file = tokio::fs::File::open(file_path).await?;
    let file_len = file.metadata().await?.len();
    let stream = ReaderStream::new(file);
    let body = reqwest::Body::wrap_stream(stream);

    let resp = http
        .put(upload_url)
        .header("Content-Type", "application/octet-stream")
        .header("Content-Length", file_len)
        .body(body)
        .send()
        .await?;

    let status = resp.status().as_u16();
    if !(200..300).contains(&status) {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("上传失败 (HTTP {status}): {body}");
    }

    let confirm_resp = http
        .post(verify_url)
        .header("Accept", "application/vnd.cnb.api+json")
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;

    let confirm_status = confirm_resp.status().as_u16();
    if confirm_status != 200 {
        let body = confirm_resp.text().await.unwrap_or_default();
        anyhow::bail!("上传确认失败 (HTTP {confirm_status}): {body}");
    }

    Ok(())
}

/// 校验文件路径有效性，返回文件名和文件大小
pub fn validate_file(file_path: &str) -> anyhow::Result<(&str, i64)> {
    let path = Path::new(file_path);
    if !path.is_file() {
        anyhow::bail!("{file_path} 不是有效的文件");
    }
    let metadata = std::fs::metadata(path)?;
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
    Ok((file_name, i64::try_from(metadata.len())?))
}
