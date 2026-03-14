//! cnb commit asset-stats 子命令 - 统计 Commit 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::{format_bytes, format_rfc3339};
use cnb_tui::{Column, Table};
use futures::future::try_join_all;

/// 执行 commit asset-stats 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let commits = client.list_all_commits().await?;

    let asset_lists =
        try_join_all(commits.iter().map(|c| client.get_commit_assets(&c.sha))).await?;

    if ctx.json() {
        // 构建带 sha 的附件列表
        let json: Vec<_> = commits
            .iter()
            .zip(asset_lists.iter())
            .filter(|(_, assets)| !assets.is_empty())
            .map(|(c, assets)| {
                serde_json::json!({
                    "sha": c.sha,
                    "assets": assets,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json)?);
        return Ok(());
    }

    let mut total_size: i64 = 0;
    let mut table = Table::new(vec![
        Column::new("SHA", 12),
        Column::new("ASSET NAME", 30),
        Column::new("SIZE", 15),
        Column::new("CREATED", 25),
    ]);

    for (commit, assets) in commits.iter().zip(asset_lists.iter()) {
        for asset in assets {
            total_size += asset.size_in_byte;
            table.add_row(vec![
                commit.sha[..12.min(commit.sha.len())].to_string(),
                asset.name.clone(),
                format_bytes(asset.size_in_byte),
                format_rfc3339(&asset.created_at),
            ]);
        }
    }

    table.print();
    println!("Total Size: {}", format_bytes(total_size));

    Ok(())
}
