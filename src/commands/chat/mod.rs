//! Chat AI 子命令
//!
//! 使用自然语言与 CNB OpenAPI 交互。

use anyhow::Result;
use cnb_api::client::CnbClient;
use clap::Args;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub mod agent;
pub mod stream;

/// Chat 命令参数
#[derive(Debug, Args)]
pub struct ChatArgs {
    /// 一次性模式：执行单个请求后退出（如 --do "查看 issue 列表"）
    #[arg(long, value_name = "问题")]
    pub do_: Option<String>,

    /// 禁用流式输出（适合 CI 环境）
    #[arg(long)]
    pub no_stream: bool,
}

/// 交互式 REPL 模式
///
/// 使用 rustyline 提供行编辑、历史记录、Ctrl+C/Ctrl+D 处理。
pub async fn interactive_chat(client: &CnbClient) -> Result<()> {
    eprintln!("欢迎使用 CNB OpenAPI 自然语言交互！");
    eprintln!("提示：'/exit' 或 '/bye' 退出，Ctrl+C 中断当前请求");

    let mut rl = DefaultEditor::new()?;

    // 加载历史记录（忽略文件不存在的错误）
    let history_path = dirs::data_local_dir()
        .map(|d| d.join("cnb").join("chat_history.txt"));
    if let Some(ref path) = history_path {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = rl.load_history(path);
    }

    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                if matches!(input, "/exit" | "/bye" | "exit" | "bye") {
                    eprintln!("再见！");
                    break;
                }

                rl.add_history_entry(input)?;

                // 每轮对话可被 Ctrl+C 中断
                let result = tokio::select! {
                    r = agent::run_agent(client, input, true) => r,
                    _ = tokio::signal::ctrl_c() => {
                        eprintln!("\n[已中断]");
                        Ok(())
                    }
                };

                if let Err(e) = result {
                    eprintln!("Error: {e}");
                }
                eprintln!("---");
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 在输入阶段：忽略，继续等待输入
                eprintln!();
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D：退出
                eprintln!("再见！");
                break;
            }
            Err(e) => {
                eprintln!("输入错误: {e}");
                break;
            }
        }
    }

    // 保存历史记录
    if let Some(ref path) = history_path {
        let _ = rl.save_history(path);
    }

    Ok(())
}
