//! cnb stats 子命令 - 提交统计仪表盘
//!
//! 基于本地 git log 数据，使用 ratatui 渲染：
//! - 历史提交排行榜（按用户统计）
//! - 过去 80 周提交趋势折线图

use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use clap::Args;
use cnb_tui::{TerminalGuard, info};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem};
use std::collections::HashMap;

/// 查看本地 Git 提交统计
#[derive(Debug, Args)]
pub struct StatsArgs;

impl StatsArgs {
    pub fn execute(&self) -> Result<()> {
        run()
    }
}

fn run() -> Result<()> {
    let lines = cnb_core::git::get_commits()?;

    if lines.is_empty() {
        info!("没有找到提交记录");
        return Ok(());
    }

    // 解析提交数据
    let today = Utc::now();
    let today_week = get_start_of_week(&today);

    let mut weekly_map = generate_last_weeks(today_week, 80);
    let mut user_map: HashMap<String, usize> = HashMap::new();

    for line in &lines {
        let parts: Vec<&str> = line.splitn(2, ';').collect();
        if parts.len() != 2 {
            continue;
        }
        let timestamp: i64 = match parts[0].trim_matches('\'').parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let user = parts[1].trim_matches('\'').to_string();

        *user_map.entry(user).or_insert(0) += 1;

        let commit_time = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
        let week_key = get_start_of_week(&commit_time);
        if let Some(count) = weekly_map.get_mut(&week_key) {
            *count += 1;
        }
    }

    // 准备排行榜数据
    let mut user_list: Vec<(String, usize)> = user_map.into_iter().collect();
    user_list.sort_by(|a, b| b.1.cmp(&a.1));
    let top_count = user_list.len().min(13);
    let top_users = &user_list[..top_count];

    // 准备趋势图数据
    let mut week_list: Vec<(NaiveDate, usize)> = weekly_map.into_iter().collect();
    week_list.sort_by(|a, b| a.0.cmp(&b.0));

    // 渲染 TUI
    render_dashboard(top_users, &week_list)?;

    Ok(())
}

/// 获取某一天所在周的周一日期
fn get_start_of_week<Tz: chrono::TimeZone>(dt: &DateTime<Tz>) -> NaiveDate {
    cnb_tui::time::start_of_week(dt.naive_utc().date())
}

/// 生成过去 N 周的映射
fn generate_last_weeks(today_week: NaiveDate, limit: usize) -> HashMap<NaiveDate, usize> {
    let mut map = HashMap::new();
    let mut week = today_week;
    for _ in 0..limit {
        map.insert(week, 0);
        week -= Duration::weeks(1);
    }
    map
}

/// 使用 ratatui 渲染仪表盘
fn render_dashboard(top_users: &[(String, usize)], week_data: &[(NaiveDate, usize)]) -> Result<()> {
    let items: Vec<(String, usize)> = top_users.to_vec();

    let chart_data: Vec<(f64, f64)> = week_data
        .iter()
        .enumerate()
        .map(|(i, (_, count))| {
            (
                f64::from(u32::try_from(i).unwrap_or(u32::MAX)),
                f64::from(u32::try_from(*count).unwrap_or(u32::MAX)),
            )
        })
        .collect();

    let max_y_count = u32::try_from(week_data.iter().map(|(_, c)| *c).max().unwrap_or(1).max(1))
        .unwrap_or(u32::MAX);
    let max_y = f64::from(max_y_count);

    let x_max =
        f64::from(u32::try_from(chart_data.len().saturating_sub(1).max(1)).unwrap_or(u32::MAX));

    let x_labels: Vec<String> =
        if let (Some(first), Some(last)) = (week_data.first(), week_data.last()) {
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
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(35), Constraint::Min(40)])
            .split(frame.area());

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|(user, commits)| ListItem::new(Line::raw(format!("[{commits:>4}] {user}"))))
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title(" 历史提交榜 "))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(list, chunks[0]);

        let datasets = vec![
            Dataset::default()
                .name("commits/week")
                .marker(ratatui::symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Green))
                .data(&chart_data),
        ];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 过去 80 周的提交曲线 "),
            )
            .x_axis(
                Axis::default().title("week").bounds([0.0, x_max]).labels(
                    x_labels
                        .iter()
                        .map(|s| ratatui::text::Span::raw(s.clone()))
                        .collect::<Vec<_>>(),
                ),
            )
            .y_axis(
                Axis::default()
                    .title("commits")
                    .bounds([0.0, max_y * 1.1])
                    .labels(vec![
                        ratatui::text::Span::raw("0"),
                        ratatui::text::Span::raw(max_y_count.to_string()),
                    ]),
            );

        frame.render_widget(chart, chunks[1]);
    })?;

    Ok(())
}
