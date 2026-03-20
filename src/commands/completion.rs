//! 终端命令行补全脚本生成

use std::ffi::{OsStr, OsString};
use std::io::Write;

use clap::{Arg, Args, Command};
use clap_complete::{Shell, generate};

const COMPLETION_SHELL_HELP: &str = "Shell type: {bash|zsh|fish|powershell|elvish}";
const COMPLETION_HELP_TEMPLATE: &str = "\
{about-with-newline}\
\n\
USAGE\n  {usage}\n\
\n\
FLAGS\n{options}{after-help}";
const COMPLETION_AFTER_HELP: &str =
    "\nLEARN MORE\n  Read the docs at https://cnb.wwvo.fun/completion\n\n";
const COMPLETION_HELP_MESSAGE: &str = "\
生成终端命令行补全脚本

USAGE
  cnb-rs completion -s <shell>

FLAGS
  -s, --shell <string>  Shell type: {bash|zsh|fish|powershell|elvish}

INHERITED FLAGS
  --help   Show help for command

LEARN MORE
  Use `cnb-rs <command> <subcommand> --help` for more information about a command.
  Read the docs at https://cnb.wwvo.fun/completion

";

#[derive(Copy, Clone, Debug, Args)]
pub struct CompletionArgs {
    #[arg(
        short = 's',
        long = "shell",
        value_enum,
        value_name = "string",
        hide_possible_values = true,
        help = COMPLETION_SHELL_HELP
    )]
    pub shell: Option<Shell>,
}

pub(crate) fn standalone_command() -> Command {
    Command::new("completion")
        .disable_version_flag(true)
        .about("生成终端命令行补全脚本")
        .override_usage("cnb-rs completion -s <shell>")
        .help_template(COMPLETION_HELP_TEMPLATE)
        .after_help(COMPLETION_AFTER_HELP)
        .arg(
            Arg::new("shell")
                .short('s')
                .long("shell")
                .value_name("string")
                .help(COMPLETION_SHELL_HELP)
                .hide_possible_values(true)
                .value_parser(clap::value_parser!(Shell)),
        )
}

pub fn customize_subcommand(cmd: Command) -> Command {
    cmd.about("生成终端命令行补全脚本")
        .override_usage("cnb-rs completion -s <shell>")
        .help_template(COMPLETION_HELP_TEMPLATE)
        .after_help(COMPLETION_AFTER_HELP)
        .mut_arg("domain", |arg| arg.hide(true))
        .mut_arg("repo", |arg| arg.hide(true))
        .mut_arg("json", |arg| arg.hide(true))
        .mut_arg("shell", |arg| {
            arg.value_name("string")
                .hide_possible_values(true)
                .help(COMPLETION_SHELL_HELP)
        })
}

pub fn handle_invocation(args: &[OsString]) -> bool {
    if matches_help_invocation(args) {
        print!("{}", format_help_output(help_message()));
        return true;
    }

    if !matches_completion_invocation(args) {
        return false;
    }

    let completion_args = std::iter::once(OsString::from("cnb-rs completion"))
        .chain(args.iter().skip(1).cloned())
        .collect::<Vec<_>>();

    let matches = match standalone_command().try_get_matches_from(completion_args) {
        Ok(matches) => matches,
        Err(err) => err.exit(),
    };

    if let Some(shell) = matches.get_one::<Shell>("shell").copied() {
        run(shell);
    } else {
        eprint!("{}", missing_shell_message());
        std::process::exit(2);
    }

    true
}

/// 生成指定 shell 的命令行补全脚本并输出到 stdout
pub fn run(shell: Shell) {
    let script = render_script(shell);

    let mut stdout = std::io::stdout().lock();
    if let Err(err) = stdout.write_all(script.as_bytes()) {
        if err.kind() != std::io::ErrorKind::BrokenPipe {
            eprint!("{}", crate::format_top_level_error(&err));
            std::process::exit(1);
        }
    }
}

pub fn missing_shell_message() -> &'static str {
    "error: the value for `--shell` is required\n\nUsage:  cnb-rs completion -s <shell>\n\nFlags:\n  -s, --shell string   Shell type: {bash|zsh|fish|powershell|elvish}\n\n"
}

fn matches_completion_invocation(args: &[OsString]) -> bool {
    matches!(args, [command, ..] if command == OsStr::new("completion"))
}

fn matches_help_invocation(args: &[OsString]) -> bool {
    matches!(
        args,
        [command, flag]
            if command == OsStr::new("completion")
                && (flag == OsStr::new("-h") || flag == OsStr::new("--help"))
    ) || matches!(
        args,
        [help, command]
            if help == OsStr::new("help") && command == OsStr::new("completion")
    )
}

pub(crate) fn render_script(shell: Shell) -> String {
    let mut cmd = crate::completion_generation_command();
    let bin_name = cmd.get_name().to_string();
    let mut output = Vec::new();
    generate(shell, &mut cmd, &bin_name, &mut output);

    let script = String::from_utf8_lossy(&output).into_owned();
    sanitize_completion_script(shell, &script)
}

pub(crate) fn help_message() -> &'static str {
    COMPLETION_HELP_MESSAGE
}

fn sanitize_completion_script(shell: Shell, script: &str) -> String {
    match shell {
        Shell::Bash => sanitize_bash_completion(script),
        Shell::Elvish => sanitize_elvish_completion(script),
        Shell::Fish => sanitize_fish_completion(script),
        Shell::PowerShell => sanitize_powershell_completion(script),
        Shell::Zsh => sanitize_zsh_completion(script),
        _ => script.to_owned(),
    }
}

pub(crate) fn format_help_output(help: &str) -> String {
    #[cfg(windows)]
    let mut rendered = help.replace('\n', "\r\n");
    #[cfg(not(windows))]
    let mut rendered = help.to_owned();

    #[cfg(windows)]
    let suffix = "\r\n\r\n";
    #[cfg(not(windows))]
    let suffix = "\n\n";

    if !rendered.ends_with(suffix) {
        rendered = rendered.trim_end_matches(&['\r', '\n'][..]).to_owned();
        rendered.push_str(suffix);
    }

    rendered
}

fn sanitize_bash_completion(script: &str) -> String {
    let mut sanitized = String::with_capacity(script.len());
    let mut in_completion = false;
    let mut skip_value_case = false;

    for line in script.lines() {
        if line.trim() == "cnb__rs__completion)" {
            in_completion = true;
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if in_completion && line.trim_start().starts_with("cnb__rs__") {
            in_completion = false;
        }

        if !in_completion {
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if skip_value_case {
            if line.trim() == ";;" {
                skip_value_case = false;
            }
            continue;
        }

        if line.contains("opts=\"") {
            let line = line
                .replace(" --domain", "")
                .replace(" --repo", "")
                .replace(" --json", "");
            sanitized.push_str(&line);
            sanitized.push('\n');
            continue;
        }

        let trimmed = line.trim();
        if trimmed == "--domain)" || trimmed == "--repo)" {
            skip_value_case = true;
            continue;
        }

        sanitized.push_str(line);
        sanitized.push('\n');
    }

    sanitized
}

fn sanitize_elvish_completion(script: &str) -> String {
    sanitize_block_lines(
        script,
        "'cnb-rs;completion'= {",
        "}",
        &["cand --domain ", "cand --repo ", "cand --json "],
    )
}

fn sanitize_fish_completion(script: &str) -> String {
    let mut sanitized = String::with_capacity(script.len());

    for line in script.lines() {
        let is_completion_line = line.contains("__fish_cnb_rs_using_subcommand completion");
        let is_hidden_global = line.contains(" -l domain ")
            || line.contains(" -l repo ")
            || line.contains(" -l json ");

        if is_completion_line && is_hidden_global {
            continue;
        }

        sanitized.push_str(line);
        sanitized.push('\n');
    }

    sanitized
}

pub(crate) fn sanitize_powershell_completion(script: &str) -> String {
    sanitize_block_lines(
        script,
        "'cnb-rs;completion' {",
        "}",
        &[
            "[CompletionResult]::new('--domain'",
            "[CompletionResult]::new('--repo'",
            "[CompletionResult]::new('--json'",
        ],
    )
}

fn sanitize_zsh_completion(script: &str) -> String {
    let mut sanitized = String::with_capacity(script.len());
    let mut in_completion = false;

    for line in script.lines() {
        if line.trim() == "(completion)" {
            in_completion = true;
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if !in_completion {
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if line.trim() == ";;" {
            in_completion = false;
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if line.contains("'--domain=") || line.contains("'--repo=") || line.contains("'--json[") {
            continue;
        }

        sanitized.push_str(line);
        sanitized.push('\n');
    }

    sanitized
}

fn sanitize_block_lines(
    script: &str,
    start_marker: &str,
    end_marker: &str,
    hidden_patterns: &[&str],
) -> String {
    let mut sanitized = String::with_capacity(script.len());
    let mut in_block = false;

    for line in script.lines() {
        if line.contains(start_marker) {
            in_block = true;
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if in_block && line.trim() == end_marker.trim() {
            in_block = false;
            sanitized.push_str(line);
            sanitized.push('\n');
            continue;
        }

        if in_block && hidden_patterns.iter().any(|pattern| line.contains(pattern)) {
            continue;
        }

        sanitized.push_str(line);
        sanitized.push('\n');
    }

    sanitized
}
