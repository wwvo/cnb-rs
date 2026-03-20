use std::ffi::{OsStr, OsString};

struct HelpEntry {
    name: &'static str,
    summary: &'static str,
}

const CORE_COMMANDS: &[HelpEntry] = &[
    HelpEntry {
        name: "auth",
        summary: "登录、查看状态与退出登录",
    },
    HelpEntry {
        name: "issue",
        summary: "创建、查看和管理 Issue",
    },
    HelpEntry {
        name: "pr",
        summary: "创建、查看和管理 PR",
    },
    HelpEntry {
        name: "repo",
        summary: "查看、创建与配置仓库",
    },
    HelpEntry {
        name: "release",
        summary: "查看和管理 Release",
    },
    HelpEntry {
        name: "build",
        summary: "查看、触发与管理构建",
    },
    HelpEntry {
        name: "browse",
        summary: "在浏览器中打开仓库或资源页面",
    },
    HelpEntry {
        name: "info",
        summary: "显示当前用户与仓库信息",
    },
    HelpEntry {
        name: "chat",
        summary: "使用自然语言与 CNB OpenAPI 交互",
    },
];

const REPOSITORY_COMMANDS: &[HelpEntry] = &[
    HelpEntry {
        name: "badge",
        summary: "获取和上传仓库徽章",
    },
    HelpEntry {
        name: "label",
        summary: "管理仓库标签",
    },
    HelpEntry {
        name: "member",
        summary: "管理仓库成员",
    },
    HelpEntry {
        name: "commit",
        summary: "管理 Commit 附件",
    },
    HelpEntry {
        name: "download",
        summary: "下载仓库文件",
    },
    HelpEntry {
        name: "stats",
        summary: "查看本地 Git 提交统计",
    },
    HelpEntry {
        name: "stars",
        summary: "查看仓库 Star 趋势",
    },
];

const PLATFORM_COMMANDS: &[HelpEntry] = &[
    HelpEntry {
        name: "group",
        summary: "管理组织",
    },
    HelpEntry {
        name: "registry",
        summary: "管理制品库与标签",
    },
    HelpEntry {
        name: "knowledge",
        summary: "管理知识库",
    },
    HelpEntry {
        name: "mission",
        summary: "管理任务集",
    },
    HelpEntry {
        name: "workspace",
        summary: "管理云原生工作区",
    },
    HelpEntry {
        name: "user",
        summary: "查看用户活动、粉丝与关注",
    },
    HelpEntry {
        name: "gpg-key",
        summary: "管理 GPG 密钥",
    },
];

const UTILITY_COMMANDS: &[HelpEntry] = &[
    HelpEntry {
        name: "config",
        summary: "查看和修改本地配置",
    },
    HelpEntry {
        name: "completion",
        summary: "生成终端命令行补全脚本",
    },
];

pub fn is_root_help_invocation(args: &[OsString]) -> bool {
    match args {
        [] => true,
        [arg]
            if matches_token(arg, "-h")
                || matches_token(arg, "--help")
                || matches_token(arg, "help") =>
        {
            true
        }
        [command, flag]
            if matches_token(command, "help")
                && (matches_token(flag, "-h") || matches_token(flag, "--help")) =>
        {
            true
        }
        _ => false,
    }
}

pub fn render() -> String {
    let mut output = String::new();

    push_line(&mut output, "cnb-rs");
    push_line(
        &mut output,
        "在命令行中高效管理你的 CNB 仓库、Issue、PR、Release 等资源。",
    );
    push_blank_line(&mut output);

    push_line(&mut output, "USAGE");
    push_line(&mut output, "  cnb-rs [OPTIONS] <COMMAND>");
    push_blank_line(&mut output);

    write_section(&mut output, "CORE COMMANDS", CORE_COMMANDS);
    write_section(&mut output, "REPOSITORY COMMANDS", REPOSITORY_COMMANDS);
    write_section(&mut output, "PLATFORM COMMANDS", PLATFORM_COMMANDS);
    write_section(&mut output, "UTILITY COMMANDS", UTILITY_COMMANDS);

    push_line(&mut output, "GLOBAL OPTIONS");
    push_line(
        &mut output,
        "  --repo <REPO>      指定仓库路径，如 `wwvo/cnb-rs/cnb-rs`",
    );
    push_line(
        &mut output,
        "  --domain <DOMAIN>  指定 CNB 域名，默认 `cnb.cool`",
    );
    push_line(
        &mut output,
        "  --json             以 JSON 输出，适合脚本调用",
    );
    push_line(&mut output, "  -h, --help         显示帮助");
    push_line(&mut output, "  -V, --version      显示版本");
    push_blank_line(&mut output);

    push_line(&mut output, "EXAMPLES");
    push_line(&mut output, "  $ cnb-rs auth login");
    push_line(
        &mut output,
        "  $ cnb-rs --repo wwvo/cnb-rs/cnb-rs issue list",
    );
    push_line(&mut output, "  $ cnb-rs browse -/issues");
    push_line(&mut output, "  $ cnb-rs release list --json");
    push_blank_line(&mut output);

    push_line(&mut output, "LEARN MORE");
    push_line(
        &mut output,
        "  Use `cnb-rs <command> --help` for more information about a command.",
    );
    push_line(&mut output, "  Read the docs at https://cnb.wwvo.fun");

    output
}

fn matches_token(actual: &OsString, expected: &str) -> bool {
    actual == OsStr::new(expected)
}

fn push_line(output: &mut String, line: &str) {
    output.push_str(line);
    output.push('\n');
}

fn push_blank_line(output: &mut String) {
    output.push('\n');
}

fn write_section(output: &mut String, title: &str, entries: &[HelpEntry]) {
    push_line(output, title);
    for entry in entries {
        output.push_str("  ");
        output.push_str(entry.name);
        for _ in entry.name.len()..12 {
            output.push(' ');
        }
        output.push_str(entry.summary);
        output.push('\n');
    }
    push_blank_line(output);
}

#[cfg(test)]
mod tests {
    use super::{is_root_help_invocation, render};
    use std::ffi::OsString;

    #[test]
    fn root_help_contains_major_sections() {
        let help = render();

        for section in [
            "USAGE",
            "CORE COMMANDS",
            "REPOSITORY COMMANDS",
            "PLATFORM COMMANDS",
            "UTILITY COMMANDS",
            "GLOBAL OPTIONS",
            "EXAMPLES",
            "LEARN MORE",
        ] {
            assert!(help.contains(section), "missing section: {section}");
        }

        assert!(help.contains("  pr          创建、查看和管理 PR"));
        assert!(!help.contains("  pull        "));
    }

    #[test]
    fn root_help_detects_supported_invocations() {
        assert!(is_root_help_invocation(&[]));
        assert!(is_root_help_invocation(&[OsString::from("--help")]));
        assert!(is_root_help_invocation(&[OsString::from("-h")]));
        assert!(is_root_help_invocation(&[OsString::from("help")]));
        assert!(is_root_help_invocation(&[
            OsString::from("help"),
            OsString::from("--help"),
        ]));

        assert!(!is_root_help_invocation(&[OsString::from("issue")]));
        assert!(!is_root_help_invocation(&[
            OsString::from("help"),
            OsString::from("issue"),
        ]));
    }
}
