//! cnb release create 子命令 - 创建 Release

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateReleaseRequest;
use cnb_core::context::AppContext;

/// 创建 Release
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// Release 的 Tag 名称
    #[arg(short = 't', long = "tag")]
    pub tag: String,

    /// Release 名称
    #[arg(short = 'n', long = "name")]
    pub name: String,

    /// Release 描述
    #[arg(short = 'b', long = "body", default_value = "")]
    pub body: String,

    /// 是否为最新版本
    #[arg(long = "make-latest", default_value = "false")]
    pub make_latest: String,

    /// 是否为预发布版本
    #[arg(long = "prerelease", default_value_t = false)]
    pub prerelease: bool,
}

/// 执行 release create 命令
pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = CreateReleaseRequest {
        tag_name: args.tag.clone(),
        name: args.name.clone(),
        body: args.body.clone(),
        draft: false,
        prerelease: args.prerelease,
        make_latest: args.make_latest.clone(),
        target_commitish: args.tag.clone(),
    };

    let release = client.create_release(&req).await?;
    println!(
        "{}{}/-/releases/tag/{}",
        client.base_web_url(),
        client.repo(),
        release.tag_name
    );

    Ok(())
}
