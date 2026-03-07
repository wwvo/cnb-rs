//! cnb release latest 子命令 - 查看最新 Release

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::{format_bytes, format_rfc3339};

/// 执行 release latest 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let release = client.get_latest_release().await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&release)?);
        return Ok(());
    }

    // 基本信息
    let mut flags = Vec::new();
    if release.is_latest {
        flags.push("Latest");
    }
    if release.prerelease {
        flags.push("Pre release");
    }
    if release.draft {
        flags.push("Draft");
    }
    let flag_str = if flags.is_empty() {
        String::new()
    } else {
        format!("  [{}]", flags.join(", "))
    };

    println!("{} — {}{flag_str}", release.tag_name, release.name);

    let published = format_rfc3339(&release.published_at);
    println!("发布时间: {published}");

    let author_name = release
        .author
        .as_ref()
        .map_or_else(String::new, |a| a.username.clone());
    if !author_name.is_empty() {
        println!("作者: {author_name}");
    }

    // 附件摘要
    if !release.assets.is_empty() {
        let total_size: i64 = release.assets.iter().map(|a| a.size).sum();
        println!(
            "附件: {} 个文件, 共 {}",
            release.assets.len(),
            format_bytes(total_size)
        );
    }

    Ok(())
}
