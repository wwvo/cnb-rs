//! cnb release asset-stats 子命令 - 统计 Release 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;

/// 执行 release asset-stats 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let releases = client.list_all_releases().await?;

    let mut total_size: i64 = 0;

    println!(
        "{:<15} {:<15} {:<20} {:<25}",
        "Name", "TAG NAME", "ASSET SIZE", "PUBLISHED"
    );

    for release in &releases {
        if release.assets.is_empty() {
            continue;
        }
        let mut release_size: i64 = 0;
        for asset in &release.assets {
            total_size += asset.size;
            release_size += asset.size;
        }
        let published = parse_rfc3339(&release.published_at);
        println!(
            "{:<15} {:<15} {:<20} {:<25}",
            release.name,
            release.tag_name,
            format_bytes(release_size),
            published
        );
    }

    println!("Total Size: {} ({})", total_size, format_bytes(total_size));

    Ok(())
}

/// 格式化字节数为人类可读格式
fn format_bytes(bytes: i64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{bytes} B")
    } else {
        format!("{size:.1} {}", units[unit_idx])
    }
}

/// 解析 RFC3339 时间为可读格式
fn parse_rfc3339(s: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}
