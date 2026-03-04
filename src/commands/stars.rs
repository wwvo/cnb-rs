//! cnb stars 子命令 - Star 累积趋势折线图
//!
//! 获取仓库所有 Star 用户数据，按周聚合生成累积趋势图。

use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use cnb_core::context::AppContext;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};
use ratatui::Terminal;
use std::collections::HashMap;
use std::io;

/// 执行 stars 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let star_users = client.list_star_users().await?;

    if star_users.total == 0 {
        println!("该仓库还没有 Star");
        return Ok(());
    }

    // 解析第一个 Star 的时间，确定起始周
    let first_star_time = parse_star_time(&star_users.users[0].stared_at);
    let first_week = get_start_of_week(first_star_time);

    // 生成从第一个 Star 到现在的所有周
    let mut weekly_map = generate_weeks(first_week);

    // 按周聚合
    for user in &star_users.users {
        let star_time = parse_star_time(&user.stared_at);
        let week_key = get_start_of_week(star_time);
        *weekly_map.entry(week_key).or_insert(0) += 1;
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

/// 解析 Star 时间字符串
fn parse_star_time(s: &str) -> NaiveDate {
    // 格式: "2006-01-02 15:04:05"
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map(|dt| dt.date())
        .unwrap_or_else(|_| {
            // 尝试 RFC3339
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.naive_utc().date())
                .unwrap_or_default()
        })
}

/// 获取某天所在周的周一
fn get_start_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    date - Duration::days(weekday as i64)
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
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let chart_data: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, (_, v))| (i as f64, *v))
        .collect();

    let max_y = data.last().map(|(_, v)| *v).unwrap_or(1.0).max(1.0);
    let x_max = chart_data.len().saturating_sub(1).max(1) as f64;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .constraints([Constraint::Min(1)])
                .split(frame.area());

            let datasets = vec![Dataset::default()
                .name(format!("stars (total: {total})"))
                .marker(ratatui::symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Yellow))
                .data(&chart_data)];

            let x_labels: Vec<ratatui::text::Span> = if let (Some(first), Some(last)) =
                (data.first(), data.last())
            {
                vec![
                    ratatui::text::Span::raw(first.0.format("%Y-%m").to_string()),
                    ratatui::text::Span::raw(last.0.format("%Y-%m").to_string()),
                ]
            } else {
                vec![]
            };

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
                        .labels(x_labels.clone()),
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

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c')
                        if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) =>
                    {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
