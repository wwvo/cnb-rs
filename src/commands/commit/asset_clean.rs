//! cnb commit asset-clean 子命令 - 清理 Commit 附件

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::fmt::format_rfc3339;
use cnb_tui::{info, success, fail};
use futures::future::try_join_all;
use futures::stream::{self, StreamExt};

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
        info!("请指定删除策略，使用 -h 查看帮助");
        return Ok(());
    }

    let client = ctx.api_client()?;
    let commits = client.list_all_commits().await?;

    let mut assets_to_delete: Vec<AssetToDelete> = Vec::new();

    if let Some(keep_days) = args.keep_days {
        // 先同步过滤出超期 commit，再并行获取 assets
        let now = chrono::Utc::now();
        let expired: Vec<_> = commits
            .iter()
            .filter(|c| {
                chrono::DateTime::parse_from_rfc3339(&c.commit.committer.date)
                    .map(|dt| (now - dt.with_timezone(&chrono::Utc)).num_days() > i64::from(keep_days))
                    .unwrap_or(false)
            })
            .collect();

        let asset_lists = try_join_all(
            expired.iter().map(|c| client.get_commit_assets(&c.sha)),
        )
        .await?;

        for (commit, assets) in expired.iter().zip(asset_lists) {
            for asset in &assets {
                assets_to_delete.push(AssetToDelete {
                    sha: commit.sha.clone(),
                    asset_id: asset.id.clone(),
                    asset_name: asset.name.clone(),
                    created_at: asset.created_at.clone(),
                });
            }
        }
    } else if let Some(keep_num) = args.keep_num {
        // 并行获取所有 commit 的 assets，再顺序计数筛选
        let asset_lists = try_join_all(
            commits.iter().map(|c| client.get_commit_assets(&c.sha)),
        )
        .await?;

        let mut count = 0u32;
        for (commit, assets) in commits.iter().zip(asset_lists) {
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
        info!("没有需要清理的附件");
        return Ok(());
    }

    // 显示待删除列表
    for asset in &assets_to_delete {
        eprintln!("{asset}");
    }
    eprintln!();

    // 确认删除
    if !confirm_action(&format!("确认删除以上 {} 个附件？", assets_to_delete.len()), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    // 并发删除（最多 10 并发）
    stream::iter(assets_to_delete.iter())
        .for_each_concurrent(10, |asset| {
            let client = &client;
            async move {
                match client
                    .delete_commit_asset(&asset.sha, &asset.asset_id)
                    .await
                {
                    Ok(()) => success!("已删除：{asset}"),
                    Err(e) => fail!("删除失败：{asset}, 错误：{e}"),
                }
            }
        })
        .await;

    Ok(())
}
