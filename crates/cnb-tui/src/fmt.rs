//! 通用格式化工具函数

/// 格式化字节数为人类可读格式（B/KB/MB/GB/TB）
#[must_use]
pub fn format_bytes(bytes: i64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    let sign = if bytes < 0 { "-" } else { "" };
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = u128::from(bytes.unsigned_abs());
    let mut remainder = 0u128;
    let mut unit_idx = 0usize;

    while size >= 1024 && unit_idx < units.len() - 1 {
        remainder = size % 1024;
        size /= 1024;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{bytes} B")
    } else {
        let decimal = remainder * 10 / 1024;
        format!("{sign}{size}.{decimal} {}", units[unit_idx])
    }
}

/// 将 RFC3339 时间字符串格式化为可读格式（YYYY-MM-DD HH:MM:SS）
#[must_use]
pub fn format_rfc3339(s: &str) -> String {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_keeps_bytes_unit() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn format_bytes_formats_fractional_units_without_float_casts() {
        assert_eq!(format_bytes(1536), "1.5 KB");
    }
}
