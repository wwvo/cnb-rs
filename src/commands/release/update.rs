//! cnb release update 子命令 - 更新 Release

use anyhow::{bail, Result};
use clap::Parser;
use cnb_api::types::UpdateReleaseRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 更新 Release
#[derive(Debug, Parser)]
pub struct UpdateArgs {
    /// Tag 名称
    #[arg(value_name = "TAG")]
    pub tag: String,

    /// 修改 Release 名称
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,

    /// 修改 Release 描述
    #[arg(short = 'b', long = "body")]
    pub body: Option<String>,

    /// 标记为草稿
    #[arg(long)]
    pub draft: bool,

    /// 标记为预发布
    #[arg(long)]
    pub prerelease: bool,

    /// 设置最新版本标记（true/false/legacy）
    #[arg(long = "make-latest")]
    pub make_latest: Option<String>,
}

/// 执行 release update 命令
pub async fn run(ctx: &AppContext, args: &UpdateArgs) -> Result<()> {
    // 至少需要指定一项修改
    if args.name.is_none()
        && args.body.is_none()
        && !args.draft
        && !args.prerelease
        && args.make_latest.is_none()
    {
        bail!("请至少指定一项要修改的内容，使用 -h 查看帮助");
    }

    let client = ctx.api_client()?;

    // 先通过 tag 获取 release ID
    let release = client
        .get_release_by_tag(client.repo(), &args.tag)
        .await?;

    let req = UpdateReleaseRequest {
        name: args.name.clone(),
        body: args.body.clone(),
        draft: if args.draft { Some(true) } else { None },
        prerelease: if args.prerelease { Some(true) } else { None },
        make_latest: args.make_latest.clone(),
    };

    client.update_release(&release.id, &req).await?;
    success!("Release {} 已更新", args.tag);

    Ok(())
}
