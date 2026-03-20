//! cnb-rs - 一个非官方的 CNB 命令行工具
//!
//! 通过 `cnb-rs` 命令使用。

use clap::{CommandFactory, Parser};
use cnb_core::context::AppContext;

mod build_info;
mod commands;
mod root_help;

/// cnb-rs - 一个非官方的 CNB 命令行工具
#[derive(Debug, Parser)]
#[clap(
    name = "cnb-rs",
    bin_name = "cnb-rs",
    version = build_info::SHORT_VERSION,
    long_version = build_info::CLAP_LONG_VERSION,
    about = "在命令行中高效管理你的 CNB 仓库、Issue、PR、Release 等资源",
)]
struct Cli {
    /// 指定 CNB 域名，默认 `cnb.cool`
    #[arg(long, global = true)]
    domain: Option<String>,

    /// 指定仓库路径，如 `wwvo/cnb-rs/cnb-rs`
    #[arg(long, global = true)]
    repo: Option<String>,

    /// 以 JSON 输出，适合脚本调用
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    /// 登录、查看状态与退出登录
    Auth(commands::auth::AuthCommand),

    /// 获取和上传仓库徽章
    Badge(commands::badge::BadgeCommand),

    /// 在浏览器中打开仓库或资源页面
    Browse(commands::browse::BrowseArgs),

    /// 查看、触发与管理构建
    Build(commands::build::BuildCommand),

    /// 使用自然语言与 CNB `OpenAPI` 交互
    Chat(commands::chat::ChatArgs),

    /// 查看和修改本地配置
    Config(commands::config::ConfigCommand),

    /// 生成终端命令行补全脚本
    #[command(override_usage = "cnb-rs completion -s <shell>")]
    Completion(commands::completion::CompletionArgs),

    /// 显示当前用户与仓库信息
    Info,

    /// 显示版本信息（建议使用 --version）
    #[command(hide = true)]
    Version,

    /// 创建、查看和管理 Issue
    Issue(commands::issue::IssueCommand),

    /// 创建、查看和管理 Pull Request
    Pull(commands::pull::PullCommand),

    /// 查看和管理 Release
    Release(commands::release::ReleaseCommand),

    /// 查看、创建与配置仓库
    Repo(commands::repo::RepoCommand),

    /// 管理 Commit 附件
    Commit(commands::commit::CommitCommand),

    /// 下载仓库文件
    Download(commands::download::DownloadArgs),

    /// GPG 密钥管理
    #[command(name = "gpg-key")]
    GpgKey(commands::gpg_key::GpgKeyCommand),

    /// 查看本地 Git 提交统计
    Stats,

    /// 查看仓库 Star 趋势
    Stars,

    /// 管理仓库标签
    Label(commands::label::LabelCommand),

    /// 管理知识库
    Knowledge(commands::knowledge::KnowledgeCommand),

    /// 管理仓库成员
    Member(commands::member::MemberCommand),

    /// 管理任务集
    Mission(commands::mission::MissionCommand),

    /// 管理制品库与标签
    Registry(commands::registry::RegistryCommand),

    /// 管理组织
    Group(commands::group::GroupCommand),

    /// 查看用户活动、粉丝与关注
    User(commands::user::UserCommand),

    /// 管理云原生工作区
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

    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if root_help::is_root_help_invocation(&args) {
        print!("{}", root_help::render());
        return;
    }
    if commands::completion::handle_invocation(&args) {
        return;
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

fn completion_generation_command() -> clap::Command {
    let mut cmd = Cli::command();
    cmd.build();
    cmd.mut_subcommand("completion", commands::completion::customize_subcommand)
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
        Commands::Completion(args) => {
            if let Some(shell) = args.shell {
                commands::completion::run(shell);
            } else {
                eprint!("{}", commands::completion::missing_shell_message());
                std::process::exit(2);
            }
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

#[cfg(test)]
mod tests {
    use crate::commands;
    use clap_complete::Shell;

    #[test]
    fn completion_accepts_long_shell_option() {
        let Ok(matches) = commands::completion::standalone_command().try_get_matches_from([
            "cnb-rs completion",
            "--shell",
            "bash",
        ]) else {
            panic!("failed to parse completion --shell");
        };
        assert!(matches!(
            matches.get_one::<Shell>("shell"),
            Some(Shell::Bash)
        ));
    }

    #[test]
    fn completion_accepts_short_shell_option() {
        let Ok(matches) = commands::completion::standalone_command().try_get_matches_from([
            "cnb-rs completion",
            "-s",
            "zsh",
        ]) else {
            panic!("failed to parse completion -s");
        };
        assert!(matches!(
            matches.get_one::<Shell>("shell"),
            Some(Shell::Zsh)
        ));
    }

    #[test]
    fn completion_rejects_legacy_positional_shell_syntax() {
        match commands::completion::standalone_command()
            .try_get_matches_from(["cnb-rs completion", "fish"])
        {
            Ok(matches) => panic!("legacy positional syntax unexpectedly parsed: {matches:?}"),
            Err(err) => {
                let message = err.to_string();
                assert!(message.contains("unexpected argument"));
                assert!(message.contains("fish"));
            }
        }
    }

    #[test]
    fn completion_missing_shell_message_matches_expected_format() {
        assert_eq!(
            commands::completion::missing_shell_message(),
            "error: the value for `--shell` is required\n\nUsage:  cnb-rs completion -s <shell>\n\nFlags:\n  -s, --shell string   Shell type: {bash|zsh|fish|powershell|elvish}\n\n"
        );
    }

    #[test]
    fn completion_help_uses_custom_template_without_global_flags() {
        let message =
            commands::completion::format_help_output(commands::completion::help_message());

        assert!(message.contains("USAGE"));
        assert!(message.contains("FLAGS"));
        assert!(message.contains("INHERITED FLAGS"));
        assert!(message.contains("--shell <string>"));
        assert!(message.contains("Show help for command"));
        assert!(message.contains(
            "Use `cnb-rs <command> <subcommand> --help` for more information about a command."
        ));
        assert!(message.contains("https://cnb.wwvo.fun/completion"));
        assert!(!message.contains("--domain"));
        assert!(!message.contains("--repo"));
        assert!(!message.contains("--json"));
    }

    #[test]
    fn generated_completion_command_hides_global_flags_for_completion_subcommand() {
        let script = commands::completion::render_script(Shell::PowerShell);
        let Some(start) = script.find("        'cnb-rs;completion' {") else {
            panic!("completion block should exist");
        };
        let Some(end) = script[start..]
            .find("        }")
            .map(|offset| start + offset)
        else {
            panic!("completion block should end");
        };
        let block = &script[start..end];

        assert!(block.contains("--shell"));
        assert!(!block.contains("--domain"));
        assert!(!block.contains("--repo"));
        assert!(!block.contains("--json"));
    }
}
