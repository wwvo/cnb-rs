//! cnb release list 子命令 - 列出 Release

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_rfc3339;
use cnb_tui::{info, Column, Table};

/// 执行 release list 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let releases = client.list_all_releases().await?;

    if releases.is_empty() {
        info!("没有找到 Release");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("Name", 15),
        Column::new("TAG NAME", 15),
        Column::new("TYPE", 15),
        Column::new("PUBLISHED", 25),
    ]);
    for release in &releases {
        let release_type = if release.is_latest {
            "Latest"
        } else if release.prerelease {
            "Pre release"
        } else {
            ""
        };
        let published = format_rfc3339(&release.published_at);
        table.add_row(vec![
            release.name.clone(),
            release.tag_name.clone(),
            release_type.to_string(),
            published,
        ]);
    }
    table.print();

    Ok(())
}
