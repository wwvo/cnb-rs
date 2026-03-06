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
    /// 指定 CNB 域名（默认：cnb.cool）
    #[arg(long, global = true)]
    domain: Option<String>,

    /// 指定仓库路径（如：looc/git-cnb）
    #[arg(long, global = true)]
    repo: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// 认证管理
    Auth(commands::auth::AuthCommand),

    /// 使用自然语言与 CNB OpenAPI 交互
    Chat(commands::chat::ChatArgs),

    /// 配置管理
    Config(commands::config::ConfigCommand),

    /// 生成终端命令行补全脚本
    Completion {
        /// 目标 shell 类型
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// 显示仓库和用户信息
    Info,

    /// 显示版本信息
    Version,

    /// Issue 管理
    Issue(commands::issue::IssueCommand),

    /// Pull Request 管理
    Pull(commands::pull::PullCommand),

    /// Release 管理
    Release(commands::release::ReleaseCommand),

    /// Commit 管理
    Commit(commands::commit::CommitCommand),

    /// 下载仓库文件
    Download(commands::download::DownloadArgs),

    /// 提交统计仪表盘
    Stats,

    /// Star 趋势图
    Stars,

    /// 知识库管理
    Knowledge(commands::knowledge::KnowledgeCommand),

    /// 组织管理
    Group(commands::group::GroupCommand),

    /// 云原生工作区管理
    Workspace(commands::workspace::WorkspaceCommand),
}

fn main() {
    // Windows: 设置控制台编码为 UTF-8，避免中文乱码
    #[cfg(windows)]
    unsafe {
        windows_sys::Win32::System::Console::SetConsoleOutputCP(65001);
        windows_sys::Win32::System::Console::SetConsoleCP(65001);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    let rt = match rt {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = rt.block_on(async_main()) {
        // 使用 stdout 而非 stderr 输出错误，因为 Windows PowerShell 的 stderr 不支持 UTF-8
        println!("Error: {e}");
        std::process::exit(1);
    }
}

async fn async_main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let ctx = AppContext::new(cli.domain, cli.repo);

    match cli.command {
        Commands::Auth(cmd) => cmd.execute(&ctx).await,
        Commands::Chat(ref args) => {
            let client = ctx.api_client()?;
            if let Some(ref question) = args.do_ {
                commands::chat::agent::run_agent(client, question, !args.no_stream).await
            } else {
                commands::chat::interactive_chat(client).await
            }
        }
        Commands::Config(cmd) => cmd.execute(&ctx),
        Commands::Completion { shell } => {
            commands::completion::run(shell);
            Ok(())
        }
        Commands::Info => commands::info::run(&ctx).await,
        Commands::Version => {
            commands::version::run();
            Ok(())
        }
        Commands::Issue(cmd) => cmd.execute(&ctx).await,
        Commands::Pull(cmd) => cmd.execute(&ctx).await,
        Commands::Release(cmd) => cmd.execute(&ctx).await,
        Commands::Commit(cmd) => cmd.execute(&ctx).await,
        Commands::Download(ref args) => commands::download::run::run(&ctx, args).await,
        Commands::Stats => commands::stats::run().await,
        Commands::Stars => commands::stars::run(&ctx).await,
        Commands::Knowledge(cmd) => cmd.execute(&ctx).await,
        Commands::Group(cmd) => cmd.execute(&ctx).await,
        Commands::Workspace(cmd) => cmd.execute(&ctx).await,
    }
}
