//! cnb member repo-access-level 子命令 - 查看自己在仓库的权限

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

use super::repo_list::format_access_level;

/// 查看自己在仓库的权限
#[derive(Debug, Parser)]
pub struct RepoAccessLevelArgs {
    /// 包含继承权限
    #[arg(long = "include-inherit", default_value_t = false)]
    pub include_inherit: bool,
}

/// 执行 member repo-access-level 命令
pub async fn run(ctx: &AppContext, args: &RepoAccessLevelArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;
    let info = client
        .get_repo_access_level(repo, args.include_inherit)
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&info)?);
        return Ok(());
    }

    println!("权限等级: {}", format_access_level(&info.access_level));
    println!("继承:     {}", if info.inherit { "是" } else { "否" });
    println!(
        "读权限:   {}",
        if info.read_privilege { "是" } else { "否" }
    );
    println!(
        "写权限:   {}",
        if info.write_privilege { "是" } else { "否" }
    );

    Ok(())
}
