//! cnb badge get 子命令 - 获取指定徽章

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 获取指定徽章
#[derive(Debug, Parser)]
pub struct GetArgs {
    /// Commit SHA 或 `latest`
    pub sha: String,

    /// 徽章名称（如 ci/status/push）
    pub badge: String,

    /// 指定分支
    #[arg(short = 'b', long = "branch")]
    pub branch: Option<String>,

    /// 输出 SVG 格式（默认）
    #[arg(long = "svg", default_value_t = false)]
    pub svg: bool,
}

/// 执行 badge get 命令
pub async fn run(ctx: &AppContext, args: &GetArgs) -> Result<()> {
    let client = ctx.api_client()?;

    // 如果全局 --json 或未指定 --svg，根据情况选择
    if ctx.json() && !args.svg {
        let result = client
            .get_badge(&args.sha, &args.badge, args.branch.as_deref())
            .await?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    // SVG 模式
    let svg = client
        .get_badge_svg(&args.sha, &args.badge, args.branch.as_deref())
        .await?;
    print!("{svg}");

    Ok(())
}
