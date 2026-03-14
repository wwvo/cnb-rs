//! cnb repo pin 子命令 — 管理仓库墙（置顶仓库）

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::success;
use cnb_tui::table::{Column, Table};

/// 管理仓库墙（置顶仓库）
#[derive(Debug, Parser)]
pub struct PinArgs {
    #[command(subcommand)]
    pub action: PinAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum PinAction {
    /// 列出置顶仓库
    List(PinListArgs),

    /// 设置组织仓库墙
    Set(PinSetArgs),
}

/// 列出置顶仓库
#[derive(Debug, Parser)]
pub struct PinListArgs {
    /// 用户名或组织名（不指定则列出当前用户的仓库墙）
    pub owner: Option<String>,
}

/// 设置组织仓库墙
#[derive(Debug, Parser)]
pub struct PinSetArgs {
    /// 要置顶的仓库路径列表
    #[arg(required = true)]
    pub repos: Vec<String>,

    /// 目标组织
    #[arg(short, long)]
    pub group: String,
}

pub async fn run(ctx: &AppContext, args: &PinArgs) -> Result<()> {
    match &args.action {
        PinAction::List(list_args) => run_list(ctx, list_args).await,
        PinAction::Set(set_args) => run_set(ctx, set_args).await,
    }
}

async fn run_list(ctx: &AppContext, args: &PinListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let repos = if let Some(owner) = &args.owner {
        // 尝试作为组织查询，如果 404 则作为用户查询
        match client.get_pinned_repos_by_group(owner).await {
            Ok(repos) => repos,
            Err(cnb_api::error::ApiError::NotFound(_)) => {
                client.get_pinned_repos_by_user(owner).await?
            }
            Err(e) => return Err(e.into()),
        }
    } else {
        // 获取当前用户的仓库墙
        let user = client.me().await?;
        client.get_pinned_repos_by_user(&user.username).await?
    };

    if repos.is_empty() {
        info!("没有置顶仓库");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&repos)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("仓库路径", 35),
        Column::new("描述", 30),
        Column::new("⭐", 6),
        Column::new("语言", 10),
    ]);

    for repo in &repos {
        let language = repo.languages.as_ref().map_or("-".to_string(), |l| {
            if l.language.is_empty() {
                "-".to_string()
            } else {
                l.language.clone()
            }
        });

        let desc = if repo.description.len() > 28 {
            format!("{}...", &repo.description[..28])
        } else {
            repo.description.clone()
        };

        table.add_row(vec![
            repo.path.clone(),
            desc,
            repo.star_count.to_string(),
            language,
        ]);
    }

    table.print();

    Ok(())
}

async fn run_set(ctx: &AppContext, args: &PinSetArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let result = client
        .set_pinned_repos_by_group(&args.group, &args.repos)
        .await?;

    success!(
        "已更新 {} 的仓库墙（共 {} 个仓库）",
        args.group,
        result.len()
    );

    Ok(())
}
