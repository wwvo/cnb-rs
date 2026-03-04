//! CNB CLI - CNB 平台专属命令行工具
//!
//! 通过 `cnb` 或 `git cnb` 命令使用。

use clap::Parser;
use cnb_core::context::AppContext;

mod commands;

/// CNB CLI - CNB 平台专属命令行工具
#[derive(Debug, Parser)]
#[clap(name = "cnb", version, about = "CNB 平台专属命令行工具")]
struct Cli {
    /// 指定 CNB 域名（默认: cnb.cool）
    #[arg(long, global = true)]
    domain: Option<String>,

    /// 指定仓库路径（如: looc/git-cnb）
    #[arg(long, global = true)]
    repo: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// 显示仓库和用户信息
    Info,

    /// 显示版本信息
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let ctx = AppContext::new(cli.domain, cli.repo);

    match cli.command {
        Commands::Info => commands::info::run(&ctx).await,
        Commands::Version => {
            commands::version::run();
            Ok(())
        }
    }
}
