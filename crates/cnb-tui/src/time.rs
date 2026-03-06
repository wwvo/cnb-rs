//! 时间处理工具函数

use chrono::{Datelike, Duration, NaiveDate};

/// 获取指定日期所在周的周一日期
pub fn start_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    date - Duration::days(i64::from(weekday))
}
