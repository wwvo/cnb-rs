//! cnb release asset-stats 子命令 - 统计 Release 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::{format_bytes, format_rfc3339};

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
        let published = format_rfc3339(&release.published_at);
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
