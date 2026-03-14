//! 终端样式辅助函数
//!
//! 基于 crossterm 的轻量封装，提供统一的终端样式 API。

use crossterm::style::Stylize;

/// 灰色文本（用于辅助信息、思考过程等）
pub fn dim(text: &str) -> String {
    format!("{}", text.dark_grey())
}

/// 清除当前行（回到行首并擦除内容）
pub fn clear_line() {
    use crossterm::{cursor, execute, terminal};
    let _ = execute!(
        std::io::stderr(),
        cursor::MoveToColumn(0),
        terminal::Clear(terminal::ClearType::CurrentLine)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_contains_text() {
        let result = dim("hello");
        assert!(result.contains("hello"));
    }
}
