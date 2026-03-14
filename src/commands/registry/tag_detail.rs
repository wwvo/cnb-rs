//! cnb registry tag detail 子命令 - 查看标签详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查看制品标签详情
#[derive(Debug, Parser)]
pub struct TagDetailArgs {
    /// 制品类型（docker/npm/pypi/maven/helm 等）
    pub pkg_type: String,

    /// 制品名称
    pub name: String,

    /// 标签名
    pub tag: String,

    /// 制品库路径（组织/制品库）
    #[arg(short = 'r', long = "registry")]
    pub registry: String,

    /// 架构（Docker 制品时指定，如 linux/amd64）
    #[arg(long = "arch")]
    pub arch: Option<String>,
}

/// 执行 registry tag detail 命令
pub async fn run(ctx: &AppContext, args: &TagDetailArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let detail = client
        .get_package_tag_detail(
            &args.registry,
            &args.pkg_type,
            &args.name,
            &args.tag,
            args.arch.as_deref(),
        )
        .await?;

    // 标签详情结构因制品类型而异，统一 JSON 输出
    println!("{}", serde_json::to_string_pretty(&detail)?);

    Ok(())
}
