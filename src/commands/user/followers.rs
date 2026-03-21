//! cnb user followers 子命令 - 查看粉丝列表

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::{info, Column, Table};

/// 查看粉丝列表
#[derive(Debug, Parser)]
pub struct FollowersArgs {
    /// 用户名（不指定则查看当前用户）
    pub username: Option<String>,

    /// 最大数量
    #[arg(short = 'L', long = "limit", default_value_t = 100)]
    pub limit: u32,
}

/// 执行 user followers 命令
pub async fn run(ctx: &AppContext, args: &FollowersArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let username = match &args.username {
        Some(u) => u.clone(),
        None => client.me().await?.username,
    };

    let users = client.get_followers(&username, 1, args.limit).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&users)?);
        return Ok(());
    }

    if users.is_empty() {
        info!("没有找到粉丝");
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("USERNAME", 20),
        Column::new("NICKNAME", 20),
        Column::new("FOLLOWING", 10),
    ]);

    for user in &users {
        table.add_row(vec![
            user.username.clone(),
            user.nickname.clone(),
            if user.is_following {
                "✓".to_string()
            } else {
                "✗".to_string()
            },
        ]);
    }

    table.print();

    Ok(())
}
