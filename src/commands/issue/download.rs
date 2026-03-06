//! cnb issue download 子命令 - 下载 Issue 为 Markdown 文件

use anyhow::Result;
use clap::Parser;
use cnb_api::types::Issue;
use cnb_core::context::AppContext;
use cnb_tui::info;
use std::fs;
use std::io::Write;
use std::path::Path;

/// 下载 Issue 为 Markdown 文件
#[derive(Debug, Parser)]
pub struct DownloadArgs {
    /// 下载所有 Issue
    #[arg(long = "all", default_value_t = false)]
    pub all: bool,

    /// 指定下载的 Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: Option<String>,
}

/// 执行 issue download 命令
pub async fn run(ctx: &AppContext, args: &DownloadArgs) -> Result<()> {
    if !args.all && args.number.is_none() {
        anyhow::bail!("请指定 Issue 编号 (--number) 或使用 --all 下载全部");
    }

    let client = ctx.api_client()?;

    // 收集需要下载的 Issue
    let issues: Vec<Issue> = if args.all {
        // 并行获取 open 和 closed issues
        let (open, closed) = tokio::join!(
            client.list_all_issues("open"),
            client.list_all_issues("closed")
        );
        let mut all = open?;
        all.extend(closed?);
        all
    } else {
        let number = args.number.as_deref().unwrap_or_default();
        let detail = client.get_issue(number).await?;
        vec![Issue {
            number: detail.number,
            title: detail.title,
            state: detail.state,
            ..Default::default()
        }]
    };

    // 创建输出目录
    let issue_dir = Path::new(".issues");
    fs::create_dir_all(issue_dir)?;

    // 下载每个 Issue
    for issue in &issues {
        let md_path = issue_dir.join(format!("{}.md", issue.number));
        let mut file = fs::File::create(&md_path)?;

        // 并行获取 Issue 详情和评论
        let (detail_result, comments_result) = tokio::join!(
            client.get_issue(&issue.number),
            client.list_all_issue_comments(&issue.number)
        );
        let detail = detail_result?;
        let comments = comments_result?;

        // 写入 Issue 标题和内容
        writeln!(file, "# {}\n", issue.title)?;
        writeln!(file, "issue number: {}\n", detail.number)?;
        writeln!(file, "{}\n", detail.body)?;

        // 写入评论
        if !comments.is_empty() {
            writeln!(file, "## Comments\n")?;
            for comment in &comments {
                writeln!(
                    file,
                    "### Comment by {} ({})\n",
                    comment.author.nickname, comment.author.username
                )?;
                writeln!(file, "**Created:** {}  ", comment.created_at)?;
                if comment.updated_at != comment.created_at {
                    writeln!(file, "**Updated:** {}  ", comment.updated_at)?;
                }
                writeln!(file)?;
                writeln!(file, "{}\n", comment.body)?;
            }
        }

        info!("已下载 Issue #{}: {}", issue.number, issue.title.trim());
    }

    info!("下载完成，共下载了 {} 个 Issue", issues.len());

    Ok(())
}
