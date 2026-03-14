//! TUI 终端管理
//!
//! 提供 RAII 风格的终端管理器，确保 raw mode 和 alternate screen
//! 在 panic 或提前返回时也能正确恢复。

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::{self, Stdout};

/// RAII 终端管理器
///
/// 创建时自动进入 raw mode + alternate screen，
/// Drop 时自动恢复终端状态（即使 panic 也能恢复）。
pub struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    /// 初始化终端：启用 raw mode 并进入 alternate screen
    pub fn new() -> anyhow::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    /// 获取内部 Terminal 的可变引用
    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    /// 运行 TUI 事件循环，按 q 或 Ctrl+C 退出
    pub fn run_loop<F>(&mut self, render: F) -> anyhow::Result<()>
    where
        F: Fn(&mut ratatui::Frame),
    {
        loop {
            self.terminal.draw(&render)?;

            if event::poll(std::time::Duration::from_millis(100))?
                && let Event::Key(key) = event::read()?
            {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c')
                        if key
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL) =>
                    {
                        break;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = crossterm::execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}
