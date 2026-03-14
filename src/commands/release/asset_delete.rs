//! cnb release asset-delete 子命令 - 删除单个 Release 附件

use anyhow::{Result, bail};
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除 Release 附件
#[derive(Debug, Parser)]
pub struct AssetDeleteArgs {
    /// Tag 名称
    #[arg(value_name = "TAG")]
    pub tag: String,

    /// 附件名称
    #[arg(long = "asset")]
    pub asset_name: String,

    /// 确认删除（跳过交互确认）
    #[arg(long)]
    pub confirm: bool,
}

/// 执行 release asset-delete 命令
pub async fn run(ctx: &AppContext, args: &AssetDeleteArgs) -> Result<()> {
    let client = ctx.api_client()?;

    // 先通过 tag 获取 release
    let release = client.get_release_by_tag(client.repo(), &args.tag).await?;

    // 查找匹配的附件
    let asset = release.assets.iter().find(|a| a.name == args.asset_name);

    let Some(asset) = asset else {
        bail!(
            "在 Release \"{}\" 中未找到附件 \"{}\"",
            args.tag,
            args.asset_name
        )
    };

    if !confirm_action(
        &format!(
            "确认删除 Release \"{}\" 中的附件 \"{}\"？",
            args.tag, args.asset_name
        ),
        args.confirm,
    )? {
        return Ok(());
    }

    client.delete_release_asset(&release.id, &asset.id).await?;
    success!(
        "附件 \"{}\" 已从 Release \"{}\" 中删除",
        args.asset_name,
        args.tag
    );

    Ok(())
}
