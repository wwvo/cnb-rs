//! cnb-rs - 一个非官方的 CNB 命令行工具
//!
//! 通过 `cnb-rs` 命令使用。

use clap::{ColorChoice, CommandFactory, Parser};
use cnb_core::context::AppContext;
use std::io::IsTerminal;

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

    /// 显示当前用户与仓库信息
    Info(commands::info::InfoArgs),

    /// 查看、触发与管理构建
    Build(commands::build::BuildCommand),

    /// 使用自然语言与 CNB `OpenAPI` 交互
    Chat(commands::chat::ChatArgs),

    /// 查看和修改本地配置
    Config(commands::config::ConfigCommand),

    /// 生成终端命令行补全脚本
    #[command(override_usage = "cnb-rs completion -s <shell>")]
    Completion(commands::completion::CompletionArgs),

    /// 显示版本信息（建议使用 --version）
    #[command(hide = true)]
    Version(commands::version::VersionArgs),

    /// 创建、查看和管理 Issue
    Issue(commands::issue::IssueCommand),

    /// 创建、查看和管理 PR
    #[command(name = "pr")]
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
    Stats(commands::stats::StatsArgs),

    /// 查看仓库 Star 趋势
    Stars(commands::stars::StarsArgs),

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
    // Windows: 设置控制台编码为 UTF-8，避免中文乱码（使用 `windows` crate 的类型化绑定，替代手写 FFI）
    #[cfg(windows)]
    #[allow(unsafe_code)]
    unsafe {
        use windows::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};
        // SAFETY: SetConsoleOutputCP/SetConsoleCP 在进程启动早期各调用一次；65001 为 UTF-8 代码页。
        let _ = SetConsoleOutputCP(65001);
        let _ = SetConsoleCP(65001);
    }

    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if root_help::is_root_help_invocation(&args) {
        print!("{}", root_help::render());
        return;
    }
    if handle_removed_command_invocation(&args) {
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
            eprint!("{}", format_top_level_error(&e));
            std::process::exit(1);
        }
    };

    if let Err(e) = rt.block_on(async_main()) {
        eprint!("{}", format_top_level_error(&e));
        std::process::exit(1);
    }
}

pub(crate) fn format_top_level_error(error: &impl std::fmt::Display) -> String {
    render_top_level_error_message(&error.to_string(), cli_color_choice())
}

fn render_top_level_error_message(message: &str, color_choice: ColorChoice) -> String {
    #[cfg(windows)]
    let line_ending = "\r\n";
    #[cfg(not(windows))]
    let line_ending = "\n";

    let styles = clap::builder::styling::Styles::default();
    let use_color = should_color_stderr(color_choice);
    let error_label = style_text("错误", styles.get_error(), use_color);

    format!("{error_label}: {message}{line_ending}{line_ending}")
}

fn handle_removed_command_invocation(args: &[std::ffi::OsString]) -> bool {
    if matches_removed_command_invocation(args) {
        eprint!("{}", removed_command_message("pull", "pr"));
        std::process::exit(2);
    }

    false
}

fn matches_removed_command_invocation(args: &[std::ffi::OsString]) -> bool {
    matches!(
        args,
        [command, ..] if command == std::ffi::OsStr::new("pull")
    ) || matches!(
        args,
        [help, command, ..]
            if help == std::ffi::OsStr::new("help") && command == std::ffi::OsStr::new("pull")
    )
}

fn removed_command_message(command: &str, replacement: &str) -> String {
    render_removed_command_message(command, replacement, cli_color_choice())
}

fn render_removed_command_message(
    command: &str,
    replacement: &str,
    color_choice: ColorChoice,
) -> String {
    #[cfg(windows)]
    let line_ending = "\r\n";
    #[cfg(not(windows))]
    let line_ending = "\n";

    let styles = clap::builder::styling::Styles::default();
    let use_color = should_color_stderr(color_choice);
    let error_label = style_text("错误", styles.get_error(), use_color);
    let tip = style_text("提示:", styles.get_valid(), use_color);
    let usage = style_text("用法:", styles.get_usage(), use_color);
    let invalid_command = style_text(&format!("'{command}'"), styles.get_invalid(), use_color);
    let valid_command = style_text(&format!("'{replacement}'"), styles.get_valid(), use_color);
    let help = style_text("'--help'", styles.get_literal(), use_color);

    format!(
        "{error_label}: 无法识别子命令 {invalid_command}{line_ending}{line_ending}  {tip} 存在相近子命令：{valid_command}{line_ending}{line_ending}{usage} cnb-rs [OPTIONS] <COMMAND>{line_ending}{line_ending}更多信息请使用 {help}。{line_ending}{line_ending}"
    )
}

fn cli_color_choice() -> ColorChoice {
    let mut cmd = Cli::command();
    cmd.build();
    cmd.get_color()
}

fn should_color_stderr(color_choice: ColorChoice) -> bool {
    match color_choice {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => std::io::stderr().is_terminal(),
    }
}

fn style_text(text: &str, style: &clap::builder::styling::Style, use_color: bool) -> String {
    if use_color {
        format!("{style}{text}{style:#}")
    } else {
        text.to_owned()
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
        Commands::Browse(ref args) => args.execute(&ctx),
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
        Commands::Info(cmd) => cmd.execute(&ctx).await,
        Commands::Version(_cmd) => {
            commands::version::VersionArgs::execute();
            Ok(())
        }
        Commands::Issue(cmd) => cmd.execute(&ctx).await,
        Commands::Pull(cmd) => cmd.execute(&ctx).await,
        Commands::Release(cmd) => cmd.execute(&ctx).await,
        Commands::Repo(cmd) => cmd.execute(&ctx).await,
        Commands::Commit(cmd) => cmd.execute(&ctx).await,
        Commands::Download(ref args) => args.execute(&ctx).await,
        Commands::GpgKey(cmd) => cmd.execute(&ctx).await,
        Commands::Stats(_cmd) => commands::stats::StatsArgs::execute(),
        Commands::Stars(cmd) => cmd.execute(&ctx).await,
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
    use crate::{Cli, Commands};
    use clap::{ColorChoice, Parser};
    use clap_complete::Shell;
    use std::ffi::OsString;

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
            "错误: 必须为 `--shell` 提供取值\n\n用法:  cnb-rs completion -s <shell>\n\n参数:\n  -s, --shell <string>   Shell 类型：{bash|zsh|fish|powershell|elvish}\n\n"
        );
    }

    #[test]
    fn completion_help_uses_custom_template_without_global_flags() {
        let message =
            commands::completion::format_help_output(commands::completion::help_message());

        assert!(message.contains("用法"));
        assert!(message.contains("参数"));
        assert!(message.contains("继承参数（选项）"));
        assert!(message.contains("--shell <string>"));
        assert!(message.contains("显示该命令的帮助"));
        assert!(message.contains(
            "使用 `cnb-rs <命令> <子命令> --help` 查看该命令的参数（含选项与位置参数）。"
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

    #[test]
    fn top_level_error_output_has_trailing_blank_line() {
        let rendered =
            super::render_top_level_error_message("当前目录不是 Git 仓库", ColorChoice::Never);

        #[cfg(windows)]
        assert_eq!(rendered, "错误: 当前目录不是 Git 仓库\r\n\r\n");
        #[cfg(not(windows))]
        assert_eq!(rendered, "错误: 当前目录不是 Git 仓库\n\n");
    }

    #[test]
    fn pr_command_maps_to_pull_module() {
        let Ok(cli) = Cli::try_parse_from(["cnb-rs", "pr", "list"]) else {
            panic!("failed to parse pr command");
        };

        assert!(matches!(cli.command, Commands::Pull(_)));
    }

    #[test]
    fn pull_command_is_no_longer_supported() {
        assert!(Cli::try_parse_from(["cnb-rs", "pull", "list"]).is_err());
    }

    #[test]
    fn removed_pull_command_invocation_is_detected() {
        assert!(super::matches_removed_command_invocation(&[
            OsString::from("pull"),
            OsString::from("list"),
        ]));
        assert!(super::matches_removed_command_invocation(&[
            OsString::from("help"),
            OsString::from("pull"),
        ]));
        assert!(!super::matches_removed_command_invocation(&[
            OsString::from("pr"),
            OsString::from("list"),
        ]));
    }

    #[test]
    fn removed_pull_command_message_matches_expected_format() {
        let rendered = super::render_removed_command_message("pull", "pr", ColorChoice::Never);

        #[cfg(windows)]
        assert_eq!(
            rendered,
            "错误: 无法识别子命令 'pull'\r\n\r\n  提示: 存在相近子命令：'pr'\r\n\r\n用法: cnb-rs [OPTIONS] <COMMAND>\r\n\r\n更多信息请使用 '--help'。\r\n\r\n"
        );
        #[cfg(not(windows))]
        assert_eq!(
            rendered,
            "错误: 无法识别子命令 'pull'\n\n  提示: 存在相近子命令：'pr'\n\n用法: cnb-rs [OPTIONS] <COMMAND>\n\n更多信息请使用 '--help'。\n\n"
        );
    }

    #[test]
    fn removed_pull_command_message_uses_clap_colors_when_forced() {
        let rendered = super::render_removed_command_message("pull", "pr", ColorChoice::Always);

        assert!(rendered.contains("\u{1b}["));
        assert!(rendered.contains("无法识别子命令"));
        assert!(rendered.contains("'pull'"));
        assert!(rendered.contains("'pr'"));
    }

    #[test]
    fn top_level_error_uses_clap_colors_when_forced() {
        let rendered =
            super::render_top_level_error_message("当前目录不是 Git 仓库", ColorChoice::Always);

        assert!(rendered.contains("\u{1b}["));
        assert!(rendered.contains("错误"));
        assert!(rendered.contains("当前目录不是 Git 仓库"));
    }
}
