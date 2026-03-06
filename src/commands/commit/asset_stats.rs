//! cnb commit asset-stats 子命令 - 统计 Commit 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_bytes;

/// 执行 commit asset-stats 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let commits = client.list_all_commits().await?;

    let mut total_size: i64 = 0;

    for commit in &commits {
        let assets = client.get_commit_assets(&commit.sha).await?;
        if assets.is_empty() {
            continue;
        }
        for asset in &assets {
            total_size += asset.size_in_byte;
        }
    }

    println!("Total Size: {} ({})", total_size, format_bytes(total_size));

    Ok(())
}
