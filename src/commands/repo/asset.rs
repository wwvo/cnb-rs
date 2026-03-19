//! cnb repo asset 子命令 — 管理仓库资产

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::table::{Column, Table};
use cnb_tui::{info, success};

/// 管理仓库资产
#[derive(Debug, Parser)]
pub struct AssetArgs {
    #[command(subcommand)]
    pub action: AssetAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum AssetAction {
    /// 列出仓库资产
    List(AssetListArgs),

    /// 删除仓库资产
    Delete(AssetDeleteArgs),
}

/// 列出仓库资产
#[derive(Debug, Parser)]
pub struct AssetListArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,

    /// 最大列出数量
    #[arg(short = 'L', long)]
    pub limit: Option<usize>,
}

/// 删除仓库资产
#[derive(Debug, Parser)]
pub struct AssetDeleteArgs {
    /// 资产 ID
    pub id: String,

    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

pub async fn run(ctx: &AppContext, args: &AssetArgs) -> Result<()> {
    match &args.action {
        AssetAction::List(list_args) => run_list(ctx, list_args).await,
        AssetAction::Delete(delete_args) => run_delete(ctx, delete_args).await,
    }
}

async fn run_list(ctx: &AppContext, args: &AssetListArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let mut assets = client.list_assets(repo_path).await?;

    if assets.is_empty() {
        info!("没有资产记录");
        return Ok(());
    }

    // 限制数量
    if let Some(limit) = args.limit {
        assets.truncate(limit);
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&assets)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("ID", 12),
        Column::new("类型", 14),
        Column::new("大小", 10),
        Column::new("路径", 35),
    ]);

    for asset in &assets {
        let record_type = asset
            .record_type
            .as_ref()
            .and_then(|v| v.as_str())
            .unwrap_or("-")
            .to_string();

        let size =
            cnb_tui::fmt::format_bytes(i64::try_from(asset.size_in_byte).unwrap_or(i64::MAX));

        table.add_row(vec![
            asset.id.clone(),
            record_type,
            size,
            asset.path.clone(),
        ]);
    }

    table.print();

    Ok(())
}

async fn run_delete(ctx: &AppContext, args: &AssetDeleteArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    if !confirm_action(&format!("确认删除资产 {} ？", args.id), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.delete_asset(repo_path, &args.id).await?;

    success!("资产已删除 ({})", args.id);

    Ok(())
}
