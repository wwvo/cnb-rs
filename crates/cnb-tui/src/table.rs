//! 表格输出组件
//!
//! 提供 UTF-8 安全的终端表格渲染，支持列宽配置和自动截断。

/// 表格列定义
pub struct Column {
    /// 列标题
    pub title: String,
    /// 列宽度（字符数）
    pub width: usize,
}

impl Column {
    /// 创建新的列定义
    pub fn new(title: &str, width: usize) -> Self {
        Self {
            title: title.to_string(),
            width,
        }
    }
}

/// 表格组件
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
}

impl Table {
    /// 创建新的表格
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
        }
    }

    /// 添加一行数据
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// 渲染并输出表格
    pub fn print(&self) {
        // 输出表头
        let header: String = self
            .columns
            .iter()
            .map(|col| pad_or_truncate(&col.title, col.width))
            .collect::<Vec<_>>()
            .join("  ");
        println!("{header}");

        // 输出数据行
        for row in &self.rows {
            let line: String = self
                .columns
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    let value = row.get(i).map_or("", String::as_str);
                    pad_or_truncate(value, col.width)
                })
                .collect::<Vec<_>>()
                .join("  ");
            println!("{line}");
        }
    }
}

/// UTF-8 安全的字符串填充或截断
///
/// 对于 CJK 字符（占 2 个显示宽度），会正确计算显示宽度。
fn pad_or_truncate(s: &str, width: usize) -> String {
    let display_width = display_width(s);
    if display_width <= width {
        // 填充空格
        let padding = width - display_width;
        format!("{s}{}", " ".repeat(padding))
    } else {
        // 截断
        truncate_to_width(s, width.saturating_sub(3), "...")
    }
}

/// 计算字符串的显示宽度（CJK 字符占 2 列）
fn display_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

/// 获取单个字符的显示宽度
fn char_width(c: char) -> usize {
    // CJK 统一表意文字、CJK 兼容表意文字、全角字符等占 2 列
    if is_wide_char(c) {
        2
    } else {
        1
    }
}

/// 判断字符是否为宽字符（CJK 等）
fn is_wide_char(c: char) -> bool {
    let cp = c as u32;
    // CJK 统一表意文字
    (0x4E00..=0x9FFF).contains(&cp)
    // CJK 统一表意文字扩展 A
    || (0x3400..=0x4DBF).contains(&cp)
    // CJK 兼容表意文字
    || (0xF900..=0xFAFF).contains(&cp)
    // CJK 统一表意文字扩展 B ~ F
    || (0x20000..=0x2FA1F).contains(&cp)
    // 全角 ASCII 和半角片假名
    || (0xFF01..=0xFF60).contains(&cp)
    || (0xFFE0..=0xFFE6).contains(&cp)
    // 日文平假名、片假名
    || (0x3040..=0x309F).contains(&cp)
    || (0x30A0..=0x30FF).contains(&cp)
    // 韩文音节
    || (0xAC00..=0xD7AF).contains(&cp)
    // CJK 符号和标点
    || (0x3000..=0x303F).contains(&cp)
}

/// 按显示宽度截断字符串，追加省略号
fn truncate_to_width(s: &str, max_width: usize, ellipsis: &str) -> String {
    let ellipsis_width = display_width(ellipsis);
    if max_width == 0 {
        return ellipsis.to_string();
    }

    let mut result = String::new();
    let mut current_width = 0;

    for c in s.chars() {
        let w = char_width(c);
        if current_width + w > max_width {
            break;
        }
        result.push(c);
        current_width += w;
    }

    // 补齐到 max_width（可能因 CJK 字符差 1 列）
    let padding = max_width - current_width;
    for _ in 0..padding {
        result.push(' ');
    }
    result.push_str(ellipsis);

    // 整体补齐到 max_width + ellipsis_width
    let total_target = max_width + ellipsis_width;
    let total_width = current_width + padding + ellipsis_width;
    if total_width < total_target {
        let extra = total_target - total_width;
        for _ in 0..extra {
            result.push(' ');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn test_display_width_cjk() {
        assert_eq!(display_width("你好"), 4);
        assert_eq!(display_width("ab 你好 cd"), 10);
    }

    #[test]
    fn test_pad_or_truncate_short() {
        let result = pad_or_truncate("hi", 10);
        assert_eq!(display_width(&result), 10);
    }

    #[test]
    fn test_pad_or_truncate_long_ascii() {
        let result = pad_or_truncate("hello world this is long", 10);
        assert!(display_width(&result) <= 10);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_pad_or_truncate_cjk() {
        let result = pad_or_truncate("这是一个很长的中文标题", 10);
        assert!(display_width(&result) <= 10);
        assert!(result.ends_with("..."));
    }
}
