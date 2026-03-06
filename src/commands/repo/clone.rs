//! cnb repo clone 子命令 — 克隆仓库到本地

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_core::config::DEFAULT_GIT_PROTOCOL;
use cnb_tui::success;

/// 克隆仓库到本地
#[derive(Debug, Parser)]
pub struct CloneArgs {
    /// 仓库路径（如 org/repo）
    pub repo: String,

    /// 克隆到指定目录
    #[arg(long)]
    pub dir: Option<String>,

    /// 浅克隆深度
    #[arg(long)]
    pub depth: Option<u32>,
}

pub async fn run(ctx: &AppContext, args: &CloneArgs) -> Result<()> {
    let domain = ctx.domain();

    // 根据 git_protocol 配置选择克隆 URL
    let protocol = ctx.config().git_protocol.as_deref().unwrap_or(DEFAULT_GIT_PROTOCOL);
    let clone_url = if protocol == "ssh" {
        format!("git@{domain}:{}.git", args.repo)
    } else {
        format!("https://{domain}/{}.git", args.repo)
    };

    // 构造 git clone 命令
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone");

    if let Some(depth) = args.depth {
        cmd.arg("--depth").arg(depth.to_string());
    }

    cmd.arg(&clone_url);

    if let Some(dir) = &args.dir {
        cmd.arg(dir);
    }

    // 执行 git clone
    let status = cmd.status().map_err(|e| anyhow::anyhow!("执行 git clone 失败：{e}"))?;

    if !status.success() {
        anyhow::bail!("git clone 退出码：{}", status.code().unwrap_or(-1));
    }

    // 确定目标目录名
    let target = args.dir.as_deref().unwrap_or_else(|| {
        args.repo.rsplit('/').next().unwrap_or(&args.repo)
    });
    success!("仓库已克隆到 ./{target}");

    Ok(())
}
