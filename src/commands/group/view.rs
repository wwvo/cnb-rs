//! cnb group view 子命令 - 查看组织详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看组织详情
#[derive(Debug, Parser)]
pub struct ViewArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 在浏览器中打开
    #[arg(short, long)]
    pub web: bool,
}

pub async fn run(ctx: &AppContext, args: &ViewArgs) -> Result<()> {
    if args.web {
        let client = ctx.api_client()?;
        let url = format!("{}{}", client.base_web_url(), args.group);
        info!("正在打开 {url}");
        AppContext::open_in_browser(&url)?;
        return Ok(());
    }

    let client = ctx.api_client()?;
    let group = client.get_group(&args.group).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&group)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("字段", 12),
        Column::new("值", 50),
    ]);

    table.add_row(vec!["名称".to_string(), group.name.clone()]);
    table.add_row(vec!["路径".to_string(), group.path.clone()]);

    if !group.description.is_empty() {
        table.add_row(vec!["描述".to_string(), group.description.clone()]);
    }
    if !group.remark.is_empty() {
        table.add_row(vec!["备注".to_string(), group.remark.clone()]);
    }
    if !group.domain.is_empty() {
        table.add_row(vec!["域名".to_string(), group.domain.clone()]);
    }
    if !group.email.is_empty() {
        table.add_row(vec!["邮箱".to_string(), group.email.clone()]);
    }
    if !group.site.is_empty() {
        table.add_row(vec!["网站".to_string(), group.site.clone()]);
    }

    table.add_row(vec![
        "成员".to_string(),
        format!("{} (总计 {})", group.member_count, group.all_member_count),
    ]);
    table.add_row(vec![
        "子组织".to_string(),
        format!("{} (总计 {})", group.sub_group_count, group.all_sub_group_count),
    ]);
    table.add_row(vec![
        "仓库".to_string(),
        format!("{} (总计 {})", group.sub_repo_count, group.all_sub_repo_count),
    ]);
    table.add_row(vec![
        "关注者".to_string(),
        group.follow_count.to_string(),
    ]);

    if !group.created_at.is_empty() {
        table.add_row(vec!["创建时间".to_string(), group.created_at.clone()]);
    }

    if group.freeze {
        table.add_row(vec!["状态".to_string(), "已冻结".to_string()]);
    }

    table.print();

    Ok(())
}
