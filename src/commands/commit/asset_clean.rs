//! cnb commit asset-clean 子命令 - 清理 Commit 附件

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_rfc3339;
use std::io::{self, BufRead, Write};

/// 清理 Commit 附件
#[derive(Debug, Parser)]
pub struct AssetCleanArgs {
    /// 删除 N 天前提交的附件
    #[arg(long = "keep-days")]
    pub keep_days: Option<u32>,

    /// 保留最近 N 个有附件的 Commit
    #[arg(long = "keep-num")]
    pub keep_num: Option<u32>,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

struct AssetToDelete {
    sha: String,
    asset_id: String,
    asset_name: String,
    created_at: String,
}

impl std::fmt::Display for AssetToDelete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sha1: {}, asset: {}, created: {}",
            self.sha, self.asset_name, format_rfc3339(&self.created_at)
        )
    }
}

/// 执行 commit asset-clean 命令
pub async fn run(ctx: &AppContext, args: &AssetCleanArgs) -> Result<()> {
    if args.keep_days.is_none() && args.keep_num.is_none() {
        println!("请指定删除策略，使用 -h 查看帮助");
        return Ok(());
    }

    let client = ctx.api_client()?;
    let commits = client.list_all_commits().await?;

    let mut assets_to_delete: Vec<AssetToDelete> = Vec::new();

    if let Some(keep_days) = args.keep_days {
        let now = chrono::Utc::now();
        for commit in &commits {
            if let Ok(committed) = chrono::DateTime::parse_from_rfc3339(&commit.commit.committer.date) {
                let committed_utc: chrono::DateTime<chrono::Utc> = committed.into();
                let days = (now - committed_utc).num_days();
                if days > keep_days as i64 {
                    let assets = client.get_commit_assets(&commit.sha).await?;
                    for asset in &assets {
                        assets_to_delete.push(AssetToDelete {
                            sha: commit.sha.clone(),
                            asset_id: asset.id.clone(),
                            asset_name: asset.name.clone(),
                            created_at: asset.created_at.clone(),
                        });
                    }
                }
            }
        }
    } else if let Some(keep_num) = args.keep_num {
        let mut count = 0u32;
        for commit in &commits {
            let assets = client.get_commit_assets(&commit.sha).await?;
            if !assets.is_empty() {
                count += 1;
            }
            if count > keep_num {
                for asset in &assets {
                    assets_to_delete.push(AssetToDelete {
                        sha: commit.sha.clone(),
                        asset_id: asset.id.clone(),
                        asset_name: asset.name.clone(),
                        created_at: asset.created_at.clone(),
                    });
                }
            }
        }
    }

    if assets_to_delete.is_empty() {
        println!("没有需要清理的附件");
        return Ok(());
    }

    // 显示待删除列表
    for asset in &assets_to_delete {
        println!("{asset}");
    }
    println!();

    // 确认删除
    if !args.yes {
        print!("确认删除以上 {} 个附件？(yes/no): ", assets_to_delete.len());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        if input.trim() != "yes" {
            println!("已取消");
            return Ok(());
        }
    }

    // 执行删除
    for asset in &assets_to_delete {
        match client
            .delete_commit_asset(&asset.sha, &asset.asset_id)
            .await
        {
            Ok(()) => println!("已删除：{asset}"),
            Err(e) => println!("删除失败：{asset}, 错误：{e}"),
        }
    }

    Ok(())
}

