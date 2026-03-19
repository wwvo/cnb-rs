//! 用户确认交互

use std::io::{self, BufRead, Write};

/// 向用户请求确认操作
///
/// 如果 `skip` 为 true，直接返回 true（用于 `--yes` 参数）。
/// 否则在终端打印提示信息，等待用户输入 "yes" 确认。
///
/// # Errors
///
/// Returns an error if flushing stdout or reading from stdin fails.
pub fn confirm_action(message: &str, skip: bool) -> anyhow::Result<bool> {
    if skip {
        return Ok(true);
    }
    print!("{message} (yes/no): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim() == "yes")
}
