//! cnb release view 子命令 - 查看 Release 详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::fmt::{format_bytes, format_rfc3339};
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看 Release 详情
#[derive(Debug, Parser)]
pub struct ViewArgs {
    /// Tag 名称
    #[arg(value_name = "TAG")]
    pub tag: String,

    /// 在浏览器中打开
    #[arg(short, long)]
    pub web: bool,
}

/// 执行 release view 命令
pub async fn run(ctx: &AppContext, args: &ViewArgs) -> Result<()> {
    let client = ctx.api_client()?;

    if args.web {
        let url = format!(
            "{}{}/-/releases/tag/{}",
            client.base_web_url(),
            client.repo(),
            args.tag
        );
        info!("正在打开 {url}");
        AppContext::open_in_browser(&url)?;
        return Ok(());
    }

    let release = client.get_release_by_tag(client.repo(), &args.tag).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&release)?);
        return Ok(());
    }

    // 基本信息
    println!("{} — {}", release.tag_name, release.name);

    let mut status_parts: Vec<&str> = Vec::new();
    if release.is_latest {
        status_parts.push("Latest");
    }
    if release.draft {
        status_parts.push("Draft");
    }
    if release.prerelease {
        status_parts.push("Pre release");
    }
    if !status_parts.is_empty() {
        println!("状态: {}", status_parts.join(" | "));
    }

    let author_name = release
        .author
        .as_ref()
        .map_or_else(String::new, |a| a.username.clone());
    if !author_name.is_empty() {
        println!("作者: {author_name}");
    }

    let published = format_rfc3339(&release.published_at);
    println!("发布时间: {published}");

    // 描述
    if !release.body.is_empty() {
        println!();
        println!("{}", release.body);
    }

    // 附件列表
    if !release.assets.is_empty() {
        println!();
        println!("附件:");
        let mut table = Table::new(vec![
            Column::new("NAME", 30),
            Column::new("SIZE", 12),
            Column::new("DOWNLOADS", 10),
        ]);
        for asset in &release.assets {
            table.add_row(vec![
                asset.name.clone(),
                format_bytes(asset.size),
                asset.download_count.to_string(),
            ]);
        }
        table.print();
    }

    Ok(())
}
