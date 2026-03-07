//! cnb registry package delete 子命令 - 删除制品

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除制品
#[derive(Debug, Parser)]
pub struct PackageDeleteArgs {
    /// 制品类型（docker/npm/pypi/maven/helm 等）
    pub pkg_type: String,

    /// 制品名称
    pub name: String,

    /// 制品库路径（组织/制品库）
    #[arg(short = 'r', long = "registry")]
    pub registry: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 registry package delete 命令
pub async fn run(ctx: &AppContext, args: &PackageDeleteArgs) -> Result<()> {
    let client = ctx.api_client()?;

    confirm_action(
        &format!("确认删除制品 {}/{}？", args.pkg_type, args.name),
        args.yes,
    )?;

    client.delete_package(&args.registry, &args.pkg_type, &args.name).await?;
    success!("制品 {}/{} 已删除", args.pkg_type, args.name);

    Ok(())
}
