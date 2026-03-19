//! cnb-rs - 一个非官方的 CNB 命令行工具
//!
//! 通过 `cnb-rs` 命令使用。

use clap::Parser;
use cnb_core::context::AppContext;

mod build_info;
mod commands;

/// cnb-rs - 一个非官方的 CNB 命令行工具
#[derive(Debug, Parser)]
#[clap(
    name = "cnb-rs",
    version = build_info::SHORT_VERSION,
    long_version = build_info::CLAP_LONG_VERSION,
    about = "一个非官方的 CNB 命令行工具",
)]
struct Cli {
    /// 指定 CNB 域名（默认：cnb.cool）
    #[arg(long, global = true)]
    domain: Option<String>,

    /// 指定仓库路径（如：looc/git-cnb）
    #[arg(long, global = true)]
    repo: Option<String>,

    /// 以 JSON 格式输出结果
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// 认证管理
    Auth(commands::auth::AuthCommand),

    /// 徽章管理
    Badge(commands::badge::BadgeCommand),

    /// 在浏览器中打开仓库页面
    Browse(commands::browse::BrowseArgs),

    /// 构建管理
    Build(commands::build::BuildCommand),

    /// 使用自然语言与 CNB `OpenAPI` 交互
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

    /// 显示版本信息（建议使用 --version）
    #[command(hide = true)]
    Version,

    /// Issue 管理
    Issue(commands::issue::IssueCommand),

    /// Pull Request 管理
    Pull(commands::pull::PullCommand),

    /// Release 管理
    Release(commands::release::ReleaseCommand),

    /// 仓库管理
    Repo(commands::repo::RepoCommand),

    /// Commit 管理
    Commit(commands::commit::CommitCommand),

    /// 下载仓库文件
    Download(commands::download::DownloadArgs),

    /// GPG 密钥管理
    #[command(name = "gpg-key")]
    GpgKey(commands::gpg_key::GpgKeyCommand),

    /// 提交统计仪表盘
    Stats,

    /// Star 趋势图
    Stars,

    /// 标签管理
    Label(commands::label::LabelCommand),

    /// 知识库管理
    Knowledge(commands::knowledge::KnowledgeCommand),

    /// 成员管理
    Member(commands::member::MemberCommand),

    /// 任务集管理
    Mission(commands::mission::MissionCommand),

    /// 制品库管理
    Registry(commands::registry::RegistryCommand),

    /// 组织管理
    Group(commands::group::GroupCommand),

    /// 用户信息
    User(commands::user::UserCommand),

    /// 云原生工作区管理
    Workspace(commands::workspace::WorkspaceCommand),
}

fn main() {
    // Windows: 设置控制台编码为 UTF-8，避免中文乱码
    #[cfg(windows)]
    // SAFETY: SetConsoleOutputCP/SetConsoleCP 是线程安全的 Windows API，
    // 在程序启动时调用一次，参数 65001 (UTF-8) 是合法的代码页值。
    #[allow(unsafe_code)]
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
        eprintln!("Error: {e}");
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
    let ctx = AppContext::new(cli.domain, cli.repo, cli.json);

    match cli.command {
        Commands::Auth(cmd) => cmd.execute(&ctx).await,
        Commands::Badge(cmd) => cmd.execute(&ctx).await,
        Commands::Browse(ref args) => commands::browse::run(&ctx, args),
        Commands::Build(cmd) => cmd.execute(&ctx).await,
        Commands::Chat(ref args) => args.execute(&ctx).await,
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
        Commands::Repo(cmd) => cmd.execute(&ctx).await,
        Commands::Commit(cmd) => cmd.execute(&ctx).await,
        Commands::Download(ref args) => args.execute(&ctx).await,
        Commands::GpgKey(cmd) => cmd.execute(&ctx).await,
        Commands::Stats => commands::stats::run(),
        Commands::Stars => commands::stars::run(&ctx).await,
        Commands::Label(cmd) => cmd.execute(&ctx).await,
        Commands::Knowledge(cmd) => cmd.execute(&ctx).await,
        Commands::Member(cmd) => cmd.execute(&ctx).await,
        Commands::Mission(cmd) => cmd.execute(&ctx).await,
        Commands::Registry(cmd) => cmd.execute(&ctx).await,
        Commands::Group(cmd) => cmd.execute(&ctx).await,
        Commands::User(cmd) => cmd.execute(&ctx).await,
        Commands::Workspace(cmd) => cmd.execute(&ctx).await,
    }
}
