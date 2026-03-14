//! cnb repo view 子命令 — 查看仓库详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看仓库详情
#[derive(Debug, Parser)]
pub struct ViewArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前 git 目录对应的仓库
    pub repo: Option<String>,

    /// 在浏览器中打开仓库页面
    #[arg(short, long)]
    pub web: bool,
}

pub async fn run(ctx: &AppContext, args: &ViewArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    // --web 模式：直接打开浏览器，不调用 API
    if args.web {
        let domain = ctx.domain();
        let url = format!("https://{domain}/{repo_path}");
        info!("正在打开 {url}");
        AppContext::open_in_browser(&url)?;
        return Ok(());
    }

    let client = ctx.api_client()?;
    let repo = client.get_repo_by_path(repo_path).await?;

    // JSON 模式
    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&repo)?);
        return Ok(());
    }

    // 表格模式
    let language = repo.languages.as_ref().map_or("-", |l| {
        if l.language.is_empty() {
            "-"
        } else {
            &l.language
        }
    });

    let updated_at = repo
        .last_updated_at
        .as_ref()
        .and_then(|t| if t.valid { Some(t.time.as_str()) } else { None })
        .unwrap_or("-");

    let visibility = repo
        .visibility_level
        .as_ref()
        .and_then(|v| v.as_str())
        .unwrap_or("-");

    let mut table = Table::new(vec![Column::new("字段", 12), Column::new("值", 60)]);

    table.add_row(vec!["名称".to_string(), repo.path.clone()]);
    if !repo.description.is_empty() {
        table.add_row(vec!["描述".to_string(), repo.description.clone()]);
    }
    table.add_row(vec!["Star".to_string(), repo.star_count.to_string()]);
    table.add_row(vec!["Fork".to_string(), repo.fork_count.to_string()]);
    table.add_row(vec!["Issue".to_string(), repo.open_issue_count.to_string()]);
    table.add_row(vec![
        "PR".to_string(),
        repo.open_pull_request_count.to_string(),
    ]);
    table.add_row(vec!["可见性".to_string(), visibility.to_string()]);
    if !repo.license.is_empty() {
        table.add_row(vec!["许可证".to_string(), repo.license.clone()]);
    }
    table.add_row(vec!["语言".to_string(), language.to_string()]);
    if !repo.web_url.is_empty() {
        table.add_row(vec!["URL".to_string(), repo.web_url.clone()]);
    }
    if !repo.created_at.is_empty() {
        table.add_row(vec!["创建时间".to_string(), repo.created_at.clone()]);
    }
    table.add_row(vec!["更新时间".to_string(), updated_at.to_string()]);

    table.print();

    Ok(())
}
