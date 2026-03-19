//! 本地 Git 操作封装
//!
//! 解析 Git remote URL，获取当前分支等信息。

use anyhow::{Context, Result, bail};
use std::process::Command;
use std::sync::{LazyLock, OnceLock};

static HTTPS_RE: LazyLock<regex_lite::Regex> = LazyLock::new(|| {
    regex_lite::Regex::new(r"^(https?)://([^/]+)/(.+?)(?:\.git)?$")
        .unwrap_or_else(|e| panic!("HTTPS 正则编译失败：{e}"))
});

static SSH_RE: LazyLock<regex_lite::Regex> = LazyLock::new(|| {
    regex_lite::Regex::new(r"^git@([^:]+):(.+?)(?:\.git)?$")
        .unwrap_or_else(|e| panic!("SSH 正则编译失败：{e}"))
});

/// 从当前 Git 仓库解析出的信息
#[derive(Debug, Clone)]
pub struct GitInfo {
    pub scheme: String,
    pub domain: String,
    pub repo: String,
}

/// 检查当前目录是否为 Git 仓库（结果缓存，CLI 执行期间不会变化）
pub fn is_git_dir() -> bool {
    static CACHED: OnceLock<bool> = OnceLock::new();
    *CACHED.get_or_init(|| {
        Command::new("git")
            .args(["rev-parse", "-q", "--git-dir"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
}

/// 从当前目录的 Git remote 解析仓库信息
pub fn parse_git_info_from_current_dir() -> Result<GitInfo> {
    if !is_git_dir() {
        bail!("当前目录不是 Git 仓库");
    }

    let output = Command::new("git")
        .args(["remote", "-v"])
        .output()
        .context("执行 git remote -v 失败")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        bail!("没有找到 Git remote");
    }

    let fetch_remotes = parse_fetch_remotes(&lines);
    if fetch_remotes.is_empty() {
        bail!("没有找到可用的 fetch remote");
    }

    let preferred_remote = current_branch_remote_name();
    let remote_url = select_remote_url(&fetch_remotes, preferred_remote.as_deref())
        .ok_or_else(|| anyhow::anyhow!("无法解析 Git remote URL"))?;

    parse_git_url(remote_url)
}

fn parse_fetch_remotes<'a>(lines: &'a [&'a str]) -> Vec<(&'a str, &'a str)> {
    lines
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[2] == "(fetch)" {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        })
        .collect()
}

fn select_remote_url<'a>(
    fetch_remotes: &'a [(&'a str, &'a str)],
    preferred_remote: Option<&str>,
) -> Option<&'a str> {
    preferred_remote
        .and_then(|preferred| remote_url_by_name(fetch_remotes, preferred))
        .or_else(|| {
            ["upstream", "cnb", "origin"]
                .into_iter()
                .find_map(|name| remote_url_by_name(fetch_remotes, name))
        })
        .or_else(|| fetch_remotes.first().map(|(_, url)| *url))
}

fn remote_url_by_name<'a>(
    fetch_remotes: &'a [(&'a str, &'a str)],
    remote_name: &str,
) -> Option<&'a str> {
    fetch_remotes
        .iter()
        .find(|(name, _)| *name == remote_name)
        .map(|(_, url)| *url)
}

fn current_branch_remote_name() -> Option<String> {
    let branch = current_branch().ok()?;
    if branch.is_empty() || branch == "HEAD" {
        return None;
    }

    let key = format!("branch.{branch}.remote");
    let output = Command::new("git")
        .args(["config", "--get", &key])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let remote = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if remote.is_empty() {
        None
    } else {
        Some(remote)
    }
}

/// 解析 Git remote URL（支持 HTTPS 和 SSH 格式）
///
/// 支持格式：
/// - `https://cnb.cool/looc/git-cnb`
/// - `https://cnb.cool/looc/git-cnb.git`
/// - `https://user:token@cnb.cool/looc/git-cnb.git`
/// - `git@cnb.cool:looc/git-cnb.git`
pub fn parse_git_url(url: &str) -> Result<GitInfo> {
    // 尝试 HTTPS 格式
    if let Some(caps) = HTTPS_RE.captures(url) {
        let scheme = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let host_part = caps.get(2).map_or("", |m| m.as_str());
        let repo = caps.get(3).map_or("", |m| m.as_str()).to_string();

        let domain = if let Some(at_pos) = host_part.find('@') {
            host_part[at_pos + 1..].to_string()
        } else {
            host_part.to_string()
        };

        return Ok(GitInfo {
            scheme,
            domain,
            repo,
        });
    }

    // 尝试 SSH 格式
    if let Some(caps) = SSH_RE.captures(url) {
        let domain = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let repo = caps.get(2).map_or("", |m| m.as_str()).to_string();

        return Ok(GitInfo {
            scheme: "https".to_string(),
            domain,
            repo,
        });
    }

    bail!("URL 格式不匹配 CNB HTTPS 或 SSH 格式：{url}")
}

/// 获取最新一次提交的标题和正文
pub fn latest_commit_message() -> Result<(String, String)> {
    if !is_git_dir() {
        bail!("当前目录不是 Git 仓库");
    }

    let output = Command::new("git")
        .args(["--no-pager", "log", "--no-merges", "-1", "--pretty=%B"])
        .output()
        .context("获取最新提交信息失败")?;

    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();
    let title = lines.first().unwrap_or(&"").to_string();
    let body = lines.get(1..).unwrap_or(&[]).join("\n");
    Ok((title, body))
}

/// 获取所有非合并提交的时间戳和作者
///
/// 返回格式：`["timestamp;author", ...]`
pub fn get_commits() -> Result<Vec<String>> {
    if !is_git_dir() {
        bail!("当前目录不是 Git 仓库");
    }

    let output = Command::new("git")
        .args([
            "log",
            "--no-merges",
            "--date=unix",
            "--pretty=format:%ad;%an",
        ])
        .output()
        .context("获取提交记录失败")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect())
}

/// 获取当前 Git 分支名
pub fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("获取当前分支失败")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(url: &str) -> GitInfo {
        match parse_git_url(url) {
            Ok(info) => info,
            Err(err) => panic!("解析 {url} 失败: {err}"),
        }
    }

    #[test]
    fn parse_https_url() {
        let info = parse_ok("https://cnb.cool/looc/git-cnb");
        assert_eq!(info.scheme, "https");
        assert_eq!(info.domain, "cnb.cool");
        assert_eq!(info.repo, "looc/git-cnb");
    }

    #[test]
    fn parse_https_url_with_git_suffix() {
        let info = parse_ok("https://cnb.cool/looc/git-cnb.git");
        assert_eq!(info.domain, "cnb.cool");
        assert_eq!(info.repo, "looc/git-cnb");
    }

    #[test]
    fn parse_https_url_with_auth() {
        let info = parse_ok("https://user:token@cnb.cool/looc/git-cnb.git");
        assert_eq!(info.domain, "cnb.cool");
        assert_eq!(info.repo, "looc/git-cnb");
    }

    #[test]
    fn parse_http_url() {
        let info = parse_ok("http://cnb.cool/group/repo");
        assert_eq!(info.scheme, "http");
        assert_eq!(info.domain, "cnb.cool");
        assert_eq!(info.repo, "group/repo");
    }

    #[test]
    fn parse_ssh_url() {
        let info = parse_ok("git@cnb.cool:looc/git-cnb.git");
        assert_eq!(info.scheme, "https");
        assert_eq!(info.domain, "cnb.cool");
        assert_eq!(info.repo, "looc/git-cnb");
    }

    #[test]
    fn parse_ssh_url_without_git_suffix() {
        let info = parse_ok("git@cnb.cool:looc/git-cnb");
        assert_eq!(info.domain, "cnb.cool");
        assert_eq!(info.repo, "looc/git-cnb");
    }

    #[test]
    fn parse_nested_group_url() {
        let info = parse_ok("https://cnb.cool/org/sub-group/repo");
        assert_eq!(info.repo, "org/sub-group/repo");
    }

    #[test]
    fn parse_invalid_url() {
        assert!(parse_git_url("not-a-url").is_err());
        assert!(parse_git_url("ftp://cnb.cool/repo").is_err());
    }

    #[test]
    fn select_remote_url_prefers_branch_remote() {
        let fetch_remotes = vec![
            ("upstream", "https://github.com/wwvo/cnb-cli-rs.git"),
            ("cnb", "https://cnb.cool/wwvo/cnb-rs/cnb-rs"),
            ("origin", "https://cnb.cool/wwvo/forked-cnb-rs"),
        ];

        let selected = select_remote_url(&fetch_remotes, Some("origin"));

        assert_eq!(selected, Some("https://cnb.cool/wwvo/forked-cnb-rs"));
    }

    #[test]
    fn select_remote_url_falls_back_to_upstream_before_cnb_and_origin() {
        let fetch_remotes = vec![
            ("github", "https://github.com/wwvo/cnb-cli-rs.git"),
            ("origin", "https://cnb.cool/wwvo/forked-cnb-rs"),
            ("cnb", "https://cnb.cool/wwvo/cnb-rs/cnb-rs"),
            ("upstream", "https://git.example.com/upstream/repo.git"),
        ];

        let selected = select_remote_url(&fetch_remotes, None);

        assert_eq!(selected, Some("https://git.example.com/upstream/repo.git"));
    }

    #[test]
    fn select_remote_url_falls_back_to_cnb_before_origin() {
        let fetch_remotes = vec![
            ("github", "https://github.com/wwvo/cnb-cli-rs.git"),
            ("origin", "https://cnb.cool/wwvo/forked-cnb-rs"),
            ("cnb", "https://cnb.cool/wwvo/cnb-rs/cnb-rs"),
        ];

        let selected = select_remote_url(&fetch_remotes, None);

        assert_eq!(selected, Some("https://cnb.cool/wwvo/cnb-rs/cnb-rs"));
    }

    #[test]
    fn select_remote_url_falls_back_to_origin() {
        let fetch_remotes = vec![
            ("github", "https://github.com/wwvo/cnb-cli-rs.git"),
            ("origin", "https://cnb.cool/wwvo/cnb-rs/cnb-rs"),
        ];

        let selected = select_remote_url(&fetch_remotes, None);

        assert_eq!(selected, Some("https://cnb.cool/wwvo/cnb-rs/cnb-rs"));
    }

    #[test]
    fn select_remote_url_falls_back_to_first_fetch_remote() {
        let fetch_remotes = vec![
            ("mirror", "https://git.example.com/group/repo.git"),
            ("backup", "https://backup.example.com/group/repo.git"),
        ];

        let selected = select_remote_url(&fetch_remotes, None);

        assert_eq!(selected, Some("https://git.example.com/group/repo.git"));
    }

    #[test]
    fn parse_fetch_remotes_ignores_push_lines() {
        let lines = vec![
            "github https://github.com/wwvo/cnb-cli-rs.git (fetch)",
            "github https://github.com/wwvo/cnb-cli-rs.git (push)",
            "origin https://cnb.cool/wwvo/cnb-rs/cnb-rs (fetch)",
        ];

        let remotes = parse_fetch_remotes(&lines);

        assert_eq!(
            remotes,
            vec![
                ("github", "https://github.com/wwvo/cnb-cli-rs.git"),
                ("origin", "https://cnb.cool/wwvo/cnb-rs/cnb-rs"),
            ]
        );
    }
}
