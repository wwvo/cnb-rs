//! cnb release list 子命令 - 列出 Release

use anyhow::Result;
use cnb_core::context::AppContext;

/// 执行 release list 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let releases = client.list_releases(1, 100).await?;

    if releases.is_empty() {
        println!("没有找到 Release");
        return Ok(());
    }

    println!(
        "{:<15} {:<15} {:<15} {:<25}",
        "Name", "TAG NAME", "TYPE", "PUBLISHED"
    );
    for release in &releases {
        let release_type = if release.is_latest {
            "Latest"
        } else if release.prerelease {
            "Pre release"
        } else {
            ""
        };
        let published = parse_rfc3339(&release.published_at);
        println!(
            "{:<15} {:<15} {:<15} {:<25}",
            release.name, release.tag_name, release_type, published
        );
    }

    Ok(())
}

/// 解析 RFC3339 时间为可读格式
fn parse_rfc3339(s: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}
