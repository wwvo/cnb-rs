//! cnb issue view 子命令 - 查看 Issue 详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看 Issue 详情
#[derive(Debug, Parser)]
pub struct ViewArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 在浏览器中打开
    #[arg(short, long)]
    pub web: bool,
}

/// 执行 issue view 命令
pub async fn run(ctx: &AppContext, args: &ViewArgs) -> Result<()> {
    // --web 模式：直接打开浏览器
    if args.web {
        let client = ctx.api_client()?;
        let url = format!(
            "{}{}/-/issues/{}",
            client.base_web_url(),
            client.repo(),
            args.number
        );
        info!("正在打开 {url}");
        AppContext::open_in_browser(&url)?;
        return Ok(());
    }

    let client = ctx.api_client()?;
    let issue = client.get_issue(&args.number).await?;

    // JSON 模式
    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&issue)?);
        return Ok(());
    }

    // 详情表格模式
    let mut table = Table::new(vec![Column::new("字段", 12), Column::new("值", 65)]);

    table.add_row(vec!["编号".to_string(), format!("#{}", issue.number)]);
    table.add_row(vec!["标题".to_string(), issue.title.clone()]);
    table.add_row(vec![
        "状态".to_string(),
        format_state(&issue.state, &issue.state_reason),
    ]);

    if !issue.priority.is_empty() {
        table.add_row(vec!["优先级".to_string(), issue.priority.clone()]);
    }

    if let Some(ref author) = issue.author {
        table.add_row(vec!["作者".to_string(), format_user(author)]);
    }

    if !issue.assignees.is_empty() {
        let assignees_str = issue
            .assignees
            .iter()
            .map(format_user)
            .collect::<Vec<_>>()
            .join(", ");
        table.add_row(vec!["处理人".to_string(), assignees_str]);
    }

    if !issue.labels.is_empty() {
        let labels_str = issue
            .labels
            .iter()
            .map(|l| l.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        table.add_row(vec!["标签".to_string(), labels_str]);
    }

    if !issue.created_at.is_empty() {
        table.add_row(vec!["创建时间".to_string(), issue.created_at.clone()]);
    }
    if !issue.updated_at.is_empty() {
        table.add_row(vec!["更新时间".to_string(), issue.updated_at.clone()]);
    }
    if !issue.started_at.is_empty() {
        table.add_row(vec!["开始日期".to_string(), issue.started_at.clone()]);
    }
    if !issue.ended_at.is_empty() {
        table.add_row(vec!["结束日期".to_string(), issue.ended_at.clone()]);
    }
    if issue.comment_count > 0 {
        table.add_row(vec!["评论数".to_string(), issue.comment_count.to_string()]);
    }

    table.print();

    // 正文内容
    if !issue.body.is_empty() {
        println!();
        println!("{}", issue.body);
    }

    Ok(())
}

/// 格式化状态显示
fn format_state(state: &str, reason: &str) -> String {
    if reason.is_empty() {
        state.to_string()
    } else {
        format!("{state} ({reason})")
    }
}

/// 格式化用户显示
fn format_user(user: &cnb_api::types::IssueUser) -> String {
    if user.nickname.is_empty() || user.nickname == user.username {
        user.username.clone()
    } else {
        format!("{} ({})", user.nickname, user.username)
    }
}
