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
    /// 显示仓库和用户信息
    Info,

    /// 显示版本信息
    Version,

    /// Issue 管理
    Issue(commands::issue::IssueCommand),
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
        Commands::Info => commands::info::run(&ctx).await,
        Commands::Version => {
            commands::version::run();
            Ok(())
        }
        Commands::Issue(cmd) => {
            use commands::issue::IssueSubcommand;
            match cmd.subcommand {
                IssueSubcommand::List(ref args) => commands::issue::list::run(&ctx, args).await,
                IssueSubcommand::Mine => commands::issue::mine::run(&ctx).await,
                IssueSubcommand::Create(ref args) => commands::issue::create::run(&ctx, args).await,
                IssueSubcommand::Close(ref args) => commands::issue::close::run(&ctx, args).await,
                IssueSubcommand::Comment(ref args) => commands::issue::comment::run(&ctx, args).await,
                IssueSubcommand::Exist(ref args) => commands::issue::exist::run(&ctx, args).await,
                IssueSubcommand::Download(ref args) => commands::issue::download::run(&ctx, args).await,
            }
        }
    }
}
