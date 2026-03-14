//! cnb registry package detail 子命令 - 查看制品详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查看制品详情
#[derive(Debug, Parser)]
pub struct PackageDetailArgs {
    /// 制品类型（docker/npm/pypi/maven/helm 等）
    pub pkg_type: String,

    /// 制品名称
    pub name: String,

    /// 制品库路径（组织/制品库）
    #[arg(short = 'r', long = "registry")]
    pub registry: String,
}

/// 执行 registry package detail 命令
pub async fn run(ctx: &AppContext, args: &PackageDetailArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let detail = client
        .get_package(&args.registry, &args.pkg_type, &args.name)
        .await?;

    // 制品详情结构因类型而异，统一使用 JSON 输出
    println!("{}", serde_json::to_string_pretty(&detail)?);

    Ok(())
}
