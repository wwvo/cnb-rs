//! cnb group member 子命令 - 组织成员管理

use anyhow::Result;
use clap::Parser;
use cnb_api::types::{GroupMemberRequest, ListGroupMembersOptions};
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::{info, success};
use cnb_tui::table::{Column, Table};

/// 组织成员管理
#[derive(Debug, Parser)]
pub struct MemberArgs {
    #[command(subcommand)]
    pub action: MemberAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum MemberAction {
    /// 列出组织成员
    List(MemberListArgs),
    /// 添加组织成员
    Add(MemberAddArgs),
    /// 更新成员权限
    Update(MemberUpdateArgs),
    /// 移除组织成员
    Remove(MemberRemoveArgs),
}

/// 列出组织成员参数
#[derive(Debug, Parser)]
pub struct MemberListArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 按角色过滤
    #[arg(short = 'r', long = "role")]
    pub role: Option<String>,

    /// 关键字搜索
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,
}

/// 添加组织成员参数
#[derive(Debug, Parser)]
pub struct MemberAddArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 用户名
    #[arg(value_name = "USERNAME")]
    pub username: String,

    /// 权限级别（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'l', long = "level", default_value = "Developer")]
    pub level: String,
}

/// 更新成员权限参数
#[derive(Debug, Parser)]
pub struct MemberUpdateArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 用户名
    #[arg(value_name = "USERNAME")]
    pub username: String,

    /// 新的权限级别（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'l', long = "level")]
    pub level: String,
}

/// 移除组织成员参数
#[derive(Debug, Parser)]
pub struct MemberRemoveArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 用户名
    #[arg(value_name = "USERNAME")]
    pub username: String,

    /// 确认移除（跳过交互确认）
    #[arg(long)]
    pub confirm: bool,
}

pub async fn run(ctx: &AppContext, args: &MemberArgs) -> Result<()> {
    match &args.action {
        MemberAction::List(a) => run_list(ctx, a).await,
        MemberAction::Add(a) => run_add(ctx, a).await,
        MemberAction::Update(a) => run_update(ctx, a).await,
        MemberAction::Remove(a) => run_remove(ctx, a).await,
    }
}

async fn run_list(ctx: &AppContext, args: &MemberListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let opts = ListGroupMembersOptions {
        page: 1,
        page_size: 100,
        role: args.role.clone(),
        search: args.search.clone(),
    };

    let members = client.list_group_members(&args.group, &opts).await?;

    if members.is_empty() {
        info!("没有找到成员");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&members)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("用户名", 20),
        Column::new("昵称", 16),
        Column::new("权限", 12),
        Column::new("加入时间", 20),
    ]);

    for m in &members {
        let level = format_access_level(&m.access_level);
        table.add_row(vec![
            m.username.clone(),
            m.nickname.clone(),
            level,
            m.join_time.clone(),
        ]);
    }

    table.print();

    Ok(())
}

async fn run_add(ctx: &AppContext, args: &MemberAddArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = GroupMemberRequest {
        access_level: args.level.clone(),
        is_outside_collaborator: false,
    };

    client
        .add_group_member(&args.group, &args.username, &req)
        .await?;
    success!(
        "已将 {} 添加为 {} 的 {} 成员",
        args.username,
        args.group,
        args.level
    );

    Ok(())
}

async fn run_update(ctx: &AppContext, args: &MemberUpdateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = GroupMemberRequest {
        access_level: args.level.clone(),
        is_outside_collaborator: false,
    };

    client
        .update_group_member(&args.group, &args.username, &req)
        .await?;
    success!(
        "已更新 {} 在 {} 的权限为 {}",
        args.username,
        args.group,
        args.level
    );

    Ok(())
}

async fn run_remove(ctx: &AppContext, args: &MemberRemoveArgs) -> Result<()> {
    if !confirm_action(
        &format!(
            "确认从组织 \"{}\" 中移除成员 \"{}\"？",
            args.group, args.username
        ),
        args.confirm,
    )? {
        return Ok(());
    }

    let client = ctx.api_client()?;
    client
        .remove_group_member(&args.group, &args.username)
        .await?;
    success!("已从 {} 中移除成员 {}", args.group, args.username);

    Ok(())
}

fn format_access_level(value: &serde_json::Value) -> String {
    // access_level 可能是字符串、对象或其他格式
    if let Some(s) = value.as_str() {
        return s.to_string();
    }
    if let Some(obj) = value.as_object() {
        // 尝试读取 name 字段
        if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
            return name.to_string();
        }
    }
    value.to_string()
}
