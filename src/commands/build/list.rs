//! cnb build list 子命令 - 列出构建记录

use anyhow::Result;
use clap::Parser;
use cnb_api::types::BuildListOptions;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

/// 列出构建记录
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 按状态过滤（pending/success/error/cancel）
    #[arg(short = 's', long = "status")]
    pub status: Option<String>,

    /// 按事件过滤
    #[arg(short = 'e', long = "event")]
    pub event: Option<String>,

    /// 按源分支过滤
    #[arg(short = 'b', long = "branch")]
    pub branch: Option<String>,

    /// 按用户过滤
    #[arg(short = 'u', long = "user")]
    pub user: Option<String>,

    /// 开始日期（YYYY-MM-DD）
    #[arg(long = "since")]
    pub since: Option<String>,

    /// 结束日期（YYYY-MM-DD）
    #[arg(long = "until")]
    pub until: Option<String>,

    /// 每页数量（默认 30）
    #[arg(short = 'n', long = "limit", default_value_t = 30)]
    pub limit: u32,
}

/// 格式化构建耗时
fn format_duration(ms: i64) -> String {
    if ms <= 0 {
        return "-".to_string();
    }
    let secs = ms / 1000;
    if secs < 60 {
        format!("{secs}s")
    } else {
        let mins = secs / 60;
        let remaining = secs % 60;
        format!("{mins}m {remaining:02}s")
    }
}

/// 执行 build list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let opts = BuildListOptions {
        page: 1,
        page_size: args.limit,
        status: args.status.clone(),
        event: args.event.clone(),
        source_ref: args.branch.clone(),
        user_name: args.user.clone(),
        create_time: args.since.clone(),
        end_time: args.until.clone(),
    };

    let result = client.get_build_logs(&opts).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    if result.data.is_empty() {
        info!("没有找到构建记录");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("SN", 25),
        Column::new("STATUS", 10),
        Column::new("EVENT", 12),
        Column::new("BRANCH", 15),
        Column::new("DURATION", 10),
        Column::new("CREATED", 20),
    ]);

    for entry in &result.data {
        table.add_row(vec![
            entry.sn.clone(),
            entry.status.clone(),
            entry.event.clone(),
            entry.source_ref.clone(),
            format_duration(entry.duration),
            entry.create_time.clone(),
        ]);
    }

    table.print();

    Ok(())
}
