//! cnb workspace list 子命令 - 列出工作区

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListWorkspacesOptions;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_rfc3339;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 列出我的工作区
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 状态过滤（running/closed/all）
    #[arg(short = 's', long = "status")]
    pub status: Option<String>,

    /// 按仓库路径过滤
    #[arg(short = 'r', long = "repo")]
    pub repo: Option<String>,

    /// 按分支过滤
    #[arg(short = 'b', long = "branch")]
    pub branch: Option<String>,
}

/// 格式化毫秒为人类可读的持续时间
fn format_duration_ms(ms: i64) -> String {
    if ms <= 0 {
        return "-".to_string();
    }
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else if minutes > 0 {
        format!("{minutes}m")
    } else {
        format!("{total_seconds}s")
    }
}

/// 执行 workspace list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let opts = ListWorkspacesOptions {
        status: args.status.clone(),
        slug: args.repo.clone(),
        branch: args.branch.clone(),
        page: 1,
        page_size: 100,
    };

    let mut all_workspaces = Vec::new();
    let mut current_opts = opts;

    loop {
        let resp = client.list_workspaces_with_options(&current_opts).await?;
        if resp.list.is_empty() {
            break;
        }
        all_workspaces.extend(resp.list);
        if all_workspaces.len() >= usize::try_from(resp.total).unwrap_or(usize::MAX) {
            break;
        }
        current_opts = ListWorkspacesOptions {
            status: args.status.clone(),
            slug: args.repo.clone(),
            branch: args.branch.clone(),
            page: current_opts.page + 1,
            page_size: 100,
        };
    }

    if all_workspaces.is_empty() {
        info!("没有找到工作区");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&all_workspaces)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("SLUG", 25),
        Column::new("BRANCH", 15),
        Column::new("STATUS", 10),
        Column::new("CREATED", 20),
        Column::new("DURATION", 10),
    ]);

    for ws in &all_workspaces {
        let created = format_rfc3339(&ws.create_time);
        let duration = format_duration_ms(ws.duration);
        table.add_row(vec![
            ws.slug.clone(),
            ws.branch.clone(),
            ws.status.clone(),
            created,
            duration,
        ]);
    }
    table.print();

    Ok(())
}
