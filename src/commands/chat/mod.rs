//! Chat AI 子命令
//!
//! 使用自然语言与 CNB OpenAPI 交互。

use std::io::{BufRead, Write};

use anyhow::Result;
use cnb_api::client::CnbClient;
use clap::Args;

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
pub async fn interactive_chat(client: &CnbClient) -> Result<()> {
    eprintln!("欢迎使用 CNB OpenAPI 自然语言交互！");
    eprintln!("提示：'/exit' 或 '/bye' 退出，Ctrl+C 中断当前请求");

    let stdin = std::io::stdin();
    let mut reader = stdin.lock();

    loop {
        // 输出提示符
        eprint!(">>> ");
        std::io::stderr().flush()?;

        let mut input = String::new();
        match reader.read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                eprintln!("\n再见！");
                break;
            }
            Ok(_) => {}
            Err(_) => {
                eprintln!();
                continue;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if matches!(input, "/exit" | "/bye" | "exit" | "bye") {
            eprintln!("再见！");
            break;
        }

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

    Ok(())
}
