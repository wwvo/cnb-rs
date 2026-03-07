//! cnb member group-user-access 子命令 - 查看指定成员在组织的权限层级

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

use super::repo_list::format_access_level;

/// 查看指定成员在组织的权限层级
#[derive(Debug, Parser)]
pub struct GroupUserAccessArgs {
    /// 用户名
    pub username: String,

    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,
}

/// 执行 member group-user-access 命令
pub async fn run(ctx: &AppContext, args: &GroupUserAccessArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let levels = client.list_member_access_level(&args.group, &args.username).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&levels)?);
        return Ok(());
    }

    if levels.is_empty() {
        eprintln!("没有找到权限信息");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("PATH", 30),
        Column::new("ACCESS LEVEL", 15),
    ]);

    for level in &levels {
        table.add_row(vec![
            level.path.clone(),
            format_access_level(&level.access_level),
        ]);
    }

    table.print();

    Ok(())
}
