//! cnb stars 子命令 - Star 累积趋势折线图
//!
//! 获取仓库所有 Star 用户数据，按周聚合生成累积趋势图。

use anyhow::Result;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use cnb_core::context::AppContext;
use cnb_tui::time::start_of_week;
use cnb_tui::{info, TerminalGuard};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};
use std::collections::HashMap;

/// 执行 stars 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let star_users = client.list_star_users().await?;

    if star_users.total == 0 {
        info!("该仓库还没有 Star");
        return Ok(());
    }

    // 解析第一个 Star 的时间，确定起始周
    let Some(first_star_time) = parse_star_time(&star_users.users[0].stared_at) else {
        info!("无法解析 Star 时间数据");
        return Ok(());
    };
    let first_week = start_of_week(first_star_time);

    // 生成从第一个 Star 到现在的所有周
    let mut weekly_map = generate_weeks(first_week);

    // 按周聚合（跳过解析失败的记录）
    for user in &star_users.users {
        if let Some(star_time) = parse_star_time(&user.stared_at) {
            let week_key = start_of_week(star_time);
            *weekly_map.entry(week_key).or_insert(0) += 1;
        }
    }

    // 排序并计算累积值
    let mut week_list: Vec<(NaiveDate, i64)> = weekly_map.into_iter().collect();
    week_list.sort_by(|a, b| a.0.cmp(&b.0));

    let mut cumulative: Vec<(NaiveDate, f64)> = Vec::new();
    let mut total = 0.0;
    for (week, count) in &week_list {
        total += *count as f64;
        cumulative.push((*week, total));
    }

    // 渲染趋势图
    render_star_chart(&cumulative, star_users.total)?;

    Ok(())
}

/// 解析 Star 时间字符串，解析失败返回 None
fn parse_star_time(s: &str) -> Option<NaiveDate> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map(|dt| dt.date())
        .ok()
        .or_else(|| {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.naive_utc().date())
                .ok()
        })
}

/// 生成从 start 到现在的所有周
fn generate_weeks(start: NaiveDate) -> HashMap<NaiveDate, i64> {
    let now = chrono::Utc::now().naive_utc().date();
    let mut map = HashMap::new();
    let mut week = start;
    while week <= now {
        map.insert(week, 0);
        week += Duration::weeks(1);
    }
    map
}

/// 使用 ratatui 渲染 Star 趋势图
fn render_star_chart(data: &[(NaiveDate, f64)], total: i64) -> Result<()> {
    let chart_data: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, (_, v))| (i as f64, *v))
        .collect();

    let max_y = data.last().map(|(_, v)| *v).unwrap_or(1.0).max(1.0);
    let x_max = chart_data.len().saturating_sub(1).max(1) as f64;

    let x_labels: Vec<String> = if let (Some(first), Some(last)) = (data.first(), data.last()) {
        vec![
            first.0.format("%Y-%m").to_string(),
            last.0.format("%Y-%m").to_string(),
        ]
    } else {
        vec![]
    };

    let mut guard = TerminalGuard::new()?;
    guard.run_loop(|frame| {
        let chunks = Layout::default()
            .constraints([Constraint::Min(1)])
            .split(frame.area());

        let datasets = vec![Dataset::default()
            .name(format!("stars (total: {total})"))
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Yellow))
            .data(&chart_data)];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Star 趋势（共 {total} 个）按 q 退出 ")),
            )
            .x_axis(
                Axis::default()
                    .title("week")
                    .bounds([0.0, x_max])
                    .labels(x_labels.iter().map(|s| ratatui::text::Span::raw(s.clone())).collect::<Vec<_>>()),
            )
            .y_axis(
                Axis::default()
                    .title("stars")
                    .bounds([0.0, max_y * 1.1])
                    .labels(vec![
                        ratatui::text::Span::raw("0"),
                        ratatui::text::Span::raw(format!("{total}")),
                    ]),
            );

        frame.render_widget(chart, chunks[0]);
    })?;

    Ok(())
}
