//! cnb release asset-clean 子命令 - 清理 Release 附件

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success, fail};

/// 清理 Release 附件
#[derive(Debug, Parser)]
pub struct AssetCleanArgs {
    /// 按 Tag 名前缀删除
    #[arg(long = "tag-prefix", alias = "tag-name-prefix")]
    pub tag_name_prefix: Option<String>,

    /// 按 Release 名前缀删除
    #[arg(long = "release-prefix", alias = "release-name-prefix")]
    pub release_name_prefix: Option<String>,

    /// 删除 N 天前发布的 Release 附件
    #[arg(long = "keep-days")]
    pub keep_days: Option<u32>,

    /// 保留最近 N 个有附件的 Release
    #[arg(long = "keep-num")]
    pub keep_num: Option<u32>,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

struct AssetToDelete {
    release_id: String,
    release_name: String,
    asset_id: String,
    asset_name: String,
    created_at: String,
}

impl std::fmt::Display for AssetToDelete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "release: {}, asset: {}, created: {}",
            self.release_name, self.asset_name, self.created_at
        )
    }
}

/// 执行 release asset-clean 命令
pub async fn run(ctx: &AppContext, args: &AssetCleanArgs) -> Result<()> {
    if args.tag_name_prefix.is_none()
        && args.release_name_prefix.is_none()
        && args.keep_days.is_none()
        && args.keep_num.is_none()
    {
        info!("请指定删除策略，使用 -h 查看帮助");
        return Ok(());
    }

    let client = ctx.api_client()?;
    let releases = client.list_all_releases().await?;

    let mut assets_to_delete: Vec<AssetToDelete> = Vec::new();

    if let Some(ref prefix) = args.tag_name_prefix {
        for release in &releases {
            if release.tag_name.starts_with(prefix) {
                collect_assets(&mut assets_to_delete, release);
            }
        }
    } else if let Some(ref prefix) = args.release_name_prefix {
        for release in &releases {
            if release.name.starts_with(prefix) {
                collect_assets(&mut assets_to_delete, release);
            }
        }
    } else if let Some(keep_days) = args.keep_days {
        let now = chrono::Utc::now();
        for release in &releases {
            if let Ok(published) = chrono::DateTime::parse_from_rfc3339(&release.published_at) {
                let published_utc: chrono::DateTime<chrono::Utc> = published.into();
                let days = (now - published_utc).num_days();
                if days > keep_days as i64 {
                    collect_assets(&mut assets_to_delete, release);
                }
            }
        }
    } else if let Some(keep_num) = args.keep_num {
        let mut count = 0u32;
        for release in &releases {
            if !release.assets.is_empty() {
                count += 1;
            }
            if count > keep_num {
                collect_assets(&mut assets_to_delete, release);
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

    // 执行删除
    for asset in &assets_to_delete {
        match client
            .delete_release_asset(&asset.release_id, &asset.asset_id)
            .await
        {
            Ok(()) => success!("已删除：{asset}"),
            Err(e) => fail!("删除失败：{asset}, 错误：{e}"),
        }
    }

    Ok(())
}

fn collect_assets(assets: &mut Vec<AssetToDelete>, release: &cnb_api::types::Release) {
    for asset in &release.assets {
        assets.push(AssetToDelete {
            release_id: release.id.clone(),
            release_name: release.name.clone(),
            asset_id: asset.id.clone(),
            asset_name: asset.name.clone(),
            created_at: asset.created_at.clone(),
        });
    }
}
