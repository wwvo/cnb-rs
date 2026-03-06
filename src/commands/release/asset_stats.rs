//! cnb release asset-stats 子命令 - 统计 Release 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::{format_bytes, format_rfc3339};
use cnb_tui::{Column, Table};

/// 执行 release asset-stats 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let releases = client.list_all_releases().await?;

    let mut total_size: i64 = 0;

    let mut table = Table::new(vec![
        Column::new("Name", 15),
        Column::new("TAG NAME", 15),
        Column::new("ASSET SIZE", 20),
        Column::new("PUBLISHED", 25),
    ]);

    for release in &releases {
        if release.assets.is_empty() {
            continue;
        }
        let release_size: i64 = release.assets.iter().map(|a| a.size).sum();
        total_size += release_size;
        let published = format_rfc3339(&release.published_at);
        table.add_row(vec![
            release.name.clone(),
            release.tag_name.clone(),
            format_bytes(release_size),
            published,
        ]);
    }
    table.print();

    println!("Total Size: {} ({})", total_size, format_bytes(total_size));

    Ok(())
}
