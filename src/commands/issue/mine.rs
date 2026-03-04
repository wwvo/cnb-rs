//! cnb issue mine 子命令 - 列出与我相关的 Issue

use anyhow::Result;
use cnb_api::types::ListIssuesOptions;
use cnb_core::context::AppContext;

/// 执行 issue mine 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;

    // 获取当前用户信息
    let me = client.me().await?;
    let repo = client.repo();

    // 获取分配给我的 Issue
    let to_me_opts = ListIssuesOptions {
        state: "open".to_string(),
        page: 1,
        page_size: 100,
        assignees: Some(me.username.clone()),
        ..Default::default()
    };
    let to_me = client.list_issues(&to_me_opts).await.unwrap_or_default();

    // 获取我创建的 Issue
    let from_me_opts = ListIssuesOptions {
        state: "open".to_string(),
        page: 1,
        page_size: 100,
        authors: Some(me.username.clone()),
        ..Default::default()
    };
    let from_me = client.list_issues(&from_me_opts).await.unwrap_or_default();

    // 合并并标记类型
    let mut results: Vec<(String, String, &str)> = Vec::new();

    // 先添加 "分配给我" 的
    for issue in &to_me {
        results.push((issue.number.clone(), issue.title.clone(), "->ME"));
    }

    // 再添加 "我创建" 的，如果已存在则标记为双向
    for issue in &from_me {
        if let Some(existing) = results.iter_mut().find(|(n, _, _)| *n == issue.number) {
            existing.2 = "ME->ME";
        } else {
            results.push((issue.number.clone(), issue.title.clone(), "ME->"));
        }
    }

    if results.is_empty() {
        println!("没有找到与我相关的 Issue");
        return Ok(());
    }

    // 表格输出
    println!("{:<15} {:<70} {:<10}", "NUMBER", "TITLE", "TYPE");
    for (number, title, issue_type) in &results {
        let title = if title.len() > 67 {
            format!("{}...", &title[..67])
        } else {
            title.clone()
        };
        println!("{:<15} {:<70} {:<10}", number, title, issue_type);
    }

    // 尝试获取同组织下 feedback 仓库的 Issue（忽略错误）
    let feedback_repo = get_feedback_repo(repo);
    if let Ok(feedback_client) = cnb_api::client::CnbClient::new(
        client.base_url(),
        client.base_web_url(),
        client.token(),
        &feedback_repo,
    ) {
        let fb_to_me_opts = ListIssuesOptions {
            state: "open".to_string(),
            page: 1,
            page_size: 100,
            assignees: Some(me.username.clone()),
            ..Default::default()
        };
        if let Ok(fb_issues) = feedback_client.list_issues(&fb_to_me_opts).await {
            for issue in &fb_issues {
                println!(
                    "{:<15} {:<70} {:<10}",
                    format!("feedback#{}", issue.number),
                    issue.title,
                    "->ME"
                );
            }
        }
    }

    Ok(())
}

/// 获取同组织下的 feedback 仓库路径
fn get_feedback_repo(repo: &str) -> String {
    let parts: Vec<&str> = repo.split('/').collect();
    let group = parts[..parts.len() - 1].join("/");
    format!("{group}/feedback")
}
