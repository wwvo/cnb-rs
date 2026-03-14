//! cnb group settings 子命令 - 查看/更新组织配置

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateGroupSettingRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;
use cnb_tui::table::{Column, Table};

/// 查看或更新组织配置
#[derive(Debug, Parser)]
pub struct SettingsArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 隐藏成员列表（0=否, 1=是）
    #[arg(long = "hide-members")]
    pub hide_members: Option<i32>,

    /// 隐藏子组织（0=否, 1=是）
    #[arg(long = "hide-sub-groups")]
    pub hide_sub_groups: Option<i32>,

    /// 组织保护（0=关闭, 1=开启）
    #[arg(long = "group-protection")]
    pub group_protection: Option<i32>,

    /// 显示私有仓库水印（0=否, 1=是）
    #[arg(long = "show-watermark")]
    pub show_watermark: Option<i32>,
}

pub async fn run(ctx: &AppContext, args: &SettingsArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let is_update = args.hide_members.is_some()
        || args.hide_sub_groups.is_some()
        || args.group_protection.is_some()
        || args.show_watermark.is_some();

    if is_update {
        let req = UpdateGroupSettingRequest {
            hide_members: args.hide_members,
            hide_sub_groups: args.hide_sub_groups,
            group_protection: args.group_protection,
            show_private_repo_watermark: args.show_watermark,
            ..Default::default()
        };
        client.update_group_setting(&args.group, &req).await?;
        success!("组织 {} 配置已更新", args.group);
        return Ok(());
    }

    // 查看模式
    let setting = client.get_group_setting(&args.group).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&setting)?);
        return Ok(());
    }

    let mut table = Table::new(vec![Column::new("配置项", 24), Column::new("值", 30)]);

    table.add_row(vec![
        "隐藏成员".to_string(),
        format_switch(setting.hide_members),
    ]);
    table.add_row(vec![
        "隐藏子组织".to_string(),
        format_switch(setting.hide_sub_groups),
    ]);
    table.add_row(vec![
        "组织保护".to_string(),
        format_switch(setting.group_protection),
    ]);
    table.add_row(vec![
        "私有仓库水印".to_string(),
        format_switch(setting.show_private_repo_watermark),
    ]);
    table.add_row(vec![
        "可显示成员（上级）".to_string(),
        format_bool(setting.can_show_members),
    ]);
    table.add_row(vec![
        "可显示子组织（上级）".to_string(),
        format_bool(setting.can_show_sub_groups),
    ]);

    if !setting.email_verification.is_empty() {
        table.add_row(vec![
            "邮箱验证".to_string(),
            setting.email_verification.join(", "),
        ]);
    }

    table.print();

    Ok(())
}

fn format_switch(value: i32) -> String {
    if value == 0 {
        "否".to_string()
    } else {
        "是".to_string()
    }
}

fn format_bool(value: bool) -> String {
    if value {
        "是".to_string()
    } else {
        "否".to_string()
    }
}
