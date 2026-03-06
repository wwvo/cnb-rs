//! cnb issue list 子命令

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListIssuesOptions;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

/// 列出仓库的 Issue
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 状态过滤（open/closed），默认 open
    #[arg(short = 's', long, default_value = "open")]
    pub state: String,

    /// 关键词搜索
    #[arg(short = 'k', long)]
    pub keyword: Option<String>,

    /// 优先级过滤（如 P0,P1）
    #[arg(short = 'p', long)]
    pub priority: Option<String>,

    /// 标签过滤（逗号分隔）
    #[arg(short = 'l', long)]
    pub labels: Option<String>,

    /// 创建者过滤
    #[arg(long)]
    pub author: Option<String>,

    /// 处理人过滤（- 表示未分配）
    #[arg(short = 'a', long)]
    pub assignee: Option<String>,

    /// 排序字段（created_at/-created_at/-updated_at/-last_acted_at）
    #[arg(long)]
    pub sort: Option<String>,

    /// 最大数量
    #[arg(short = 'L', long, default_value_t = 30)]
    pub limit: u32,

    /// 过滤 N 天内没有活动的 Issue（0 表示不过滤）
    #[arg(short = 'd', long = "stale-days", default_value_t = 0)]
    pub stale_days: u32,
}

/// 执行 issue list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    // 使用 API 过滤参数直接查询
    let opts = ListIssuesOptions {
        state: args.state.clone(),
        page: 1,
        page_size: args.limit.min(100),
        keyword: args.keyword.clone(),
        priority: args.priority.clone(),
        labels: args.labels.clone(),
        authors: args.author.clone(),
        assignees: args.assignee.clone(),
        order_by: args.sort.clone(),
        ..Default::default()
    };

    let issues = client.list_issues(&opts).await?;

    // 统一使用同一个时间基准
    let now = chrono::Utc::now();

    // stale-days 客户端过滤
    let filtered: Vec<_> = if args.stale_days > 0 {
        issues
            .into_iter()
            .filter(|issue| is_stale(&now, &issue.last_acted_at, args.stale_days))
            .collect()
    } else {
        issues
    };

    if filtered.is_empty() {
        info!("没有找到符合条件的 Issue");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&filtered)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("NUMBER", 8),
        Column::new("TITLE", 45),
        Column::new("PRIORITY", 8),
        Column::new("AUTHOR", 12),
        Column::new("LastActedAt", 22),
        Column::new("StaleDays", 9),
    ]);
    for issue in &filtered {
        let stale_days = calculate_stale_days(&now, &issue.last_acted_at);
        let author = issue
            .author
            .as_ref()
            .map_or("-".to_string(), |a| a.username.clone());
        table.add_row(vec![
            issue.number.clone(),
            issue.title.clone(),
            if issue.priority.is_empty() { "-".to_string() } else { issue.priority.clone() },
            author,
            issue.last_acted_at.clone(),
            stale_days.to_string(),
        ]);
    }
    table.print();

    Ok(())
}

/// 判断 Issue 是否超过指定天数未活动
fn is_stale(now: &chrono::DateTime<chrono::Utc>, last_acted_at: &str, stale_days: u32) -> bool {
    if stale_days == 0 {
        return false;
    }
    let days = calculate_stale_days(now, last_acted_at);
    days >= stale_days
}

/// 计算不活跃天数
fn calculate_stale_days(now: &chrono::DateTime<chrono::Utc>, last_acted_at: &str) -> u32 {
    // 解析 RFC3339 格式的时间字符串
    let Ok(last_time) = chrono::DateTime::parse_from_rfc3339(last_acted_at) else {
        return 0;
    };
    let duration = now.signed_duration_since(last_time);
    duration.num_days().max(0) as u32
}
