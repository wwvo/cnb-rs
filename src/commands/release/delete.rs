//! cnb release delete 子命令 - 删除 Release

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除 Release
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// Tag 名称
    #[arg(value_name = "TAG")]
    pub tag: String,

    /// 确认删除（跳过交互确认）
    #[arg(long)]
    pub confirm: bool,
}

/// 执行 release delete 命令
pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    if !confirm_action(
        &format!("确认删除 Release \"{}\"？此操作不可逆！", args.tag),
        args.confirm,
    )? {
        return Ok(());
    }

    let client = ctx.api_client()?;

    // 先通过 tag 获取 release ID
    let release = client
        .get_release_by_tag(client.repo(), &args.tag)
        .await?;

    client.delete_release(&release.id).await?;
    success!("Release \"{}\" 已删除", args.tag);

    Ok(())
}
