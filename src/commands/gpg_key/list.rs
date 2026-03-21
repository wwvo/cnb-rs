//! cnb gpg-key list 子命令 - 列出 GPG 密钥

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table, info};

/// 列出 GPG 密钥
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 搜索关键字
    #[arg(short = 'k', long = "keyword")]
    pub keyword: Option<String>,
}

/// 格式化邮箱列表
fn format_emails(emails: &[cnb_api::types::GpgKeyEmail]) -> String {
    if emails.is_empty() {
        return "-".to_string();
    }
    emails
        .iter()
        .map(|e| {
            let status = if e.verified { "✓" } else { "✗" };
            format!("{} ({status})", e.email)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// 格式化过期时间
fn format_expires(expired_at: &str) -> String {
    if expired_at.is_empty() || expired_at == "0001-01-01T00:00:00Z" {
        "never".to_string()
    } else if let Some(date) = expired_at.get(..10) {
        date.to_string()
    } else {
        expired_at.to_string()
    }
}

/// 执行 gpg-key list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let keys = client.list_gpg_keys(args.keyword.as_deref()).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&keys)?);
        return Ok(());
    }

    if keys.is_empty() {
        info!("没有找到 GPG 密钥");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("KEY ID", 20),
        Column::new("NAME", 20),
        Column::new("EMAILS", 30),
        Column::new("EXPIRES", 12),
        Column::new("SUBKEYS", 8),
    ]);

    for key in &keys {
        table.add_row(vec![
            key.key_id.clone(),
            key.name.clone(),
            format_emails(&key.emails),
            format_expires(&key.expired_at),
            key.subkeys.len().to_string(),
        ]);
    }

    table.print();

    Ok(())
}
