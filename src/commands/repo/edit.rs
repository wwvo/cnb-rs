//! cnb repo edit 子命令 — 编辑仓库信息

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateRepoRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 编辑仓库信息
#[derive(Debug, Parser)]
pub struct EditArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前 git 目录对应的仓库
    pub repo: Option<String>,

    /// 仓库描述
    #[arg(short, long)]
    pub description: Option<String>,

    /// 开源许可证
    #[arg(short, long)]
    pub license: Option<String>,

    /// 仓库站点 URL
    #[arg(short, long)]
    pub site: Option<String>,

    /// 主题标签（逗号分隔，如 "cli,rust,tool"）
    #[arg(short, long, value_delimiter = ',')]
    pub topics: Option<Vec<String>>,
}

pub async fn run(ctx: &AppContext, args: &EditArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.clone(),
        None => ctx.repo()?.to_string(),
    };

    // 至少需要指定一个要修改的字段
    if args.description.is_none()
        && args.license.is_none()
        && args.site.is_none()
        && args.topics.is_none()
    {
        anyhow::bail!("请至少指定一个要修改的字段（--description, --license, --site, --topics）");
    }

    let req = UpdateRepoRequest {
        description: args.description.clone(),
        license: args.license.clone(),
        site: args.site.clone(),
        topics: args.topics.clone(),
    };

    let client = ctx.api_client()?;
    client.update_repo(&repo_path, &req).await?;

    success!("仓库信息已更新 ({repo_path})");

    Ok(())
}
