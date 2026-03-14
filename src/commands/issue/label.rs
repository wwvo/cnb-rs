//! cnb issue label 子命令 - 管理 Issue 标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::IssueLabelRequest;
use cnb_core::context::AppContext;
use cnb_tui::table::{Column, Table};
use cnb_tui::{info, success};

/// Issue 标签管理
#[derive(Debug, Parser)]
pub struct LabelArgs {
    #[command(subcommand)]
    pub action: LabelAction,
}

#[derive(Debug, clap::Subcommand)]
pub enum LabelAction {
    /// 列出 Issue 的标签
    List(LabelListArgs),

    /// 添加标签
    Add(LabelModifyArgs),

    /// 替换所有标签
    Set(LabelModifyArgs),

    /// 删除指定标签
    Remove(LabelRemoveArgs),

    /// 清空所有标签
    Clear(LabelClearArgs),
}

/// 列出标签参数
#[derive(Debug, Parser)]
pub struct LabelListArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,
}

/// 添加/设置标签参数
#[derive(Debug, Parser)]
pub struct LabelModifyArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 标签名称（逗号分隔）
    #[arg(value_name = "LABELS", value_delimiter = ',')]
    pub labels: Vec<String>,
}

/// 删除标签参数
#[derive(Debug, Parser)]
pub struct LabelRemoveArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,

    /// 要删除的标签名称
    #[arg(value_name = "NAME")]
    pub name: String,
}

/// 清空标签参数
#[derive(Debug, Parser)]
pub struct LabelClearArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,
}

/// 执行 label 命令
pub async fn run(ctx: &AppContext, args: &LabelArgs) -> Result<()> {
    match &args.action {
        LabelAction::List(a) => run_list(ctx, a).await,
        LabelAction::Add(a) => run_add(ctx, a).await,
        LabelAction::Set(a) => run_set(ctx, a).await,
        LabelAction::Remove(a) => run_remove(ctx, a).await,
        LabelAction::Clear(a) => run_clear(ctx, a).await,
    }
}

/// 列出 Issue 标签
async fn run_list(ctx: &AppContext, args: &LabelListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let labels = client.list_issue_labels(&args.number).await?;

    if labels.is_empty() {
        info!("Issue #{} 没有标签", args.number);
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&labels)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("名称", 20),
        Column::new("颜色", 10),
        Column::new("描述", 40),
    ]);
    for label in &labels {
        table.add_row(vec![
            label.name.clone(),
            label.color.clone(),
            label.description.clone(),
        ]);
    }
    table.print();

    Ok(())
}

/// 添加标签
async fn run_add(ctx: &AppContext, args: &LabelModifyArgs) -> Result<()> {
    let labels = clean_labels(&args.labels)?;
    let client = ctx.api_client()?;

    let req = IssueLabelRequest { labels };
    client.add_issue_labels(&args.number, &req).await?;
    success!("Issue #{} 标签已添加", args.number);

    Ok(())
}

/// 替换所有标签
async fn run_set(ctx: &AppContext, args: &LabelModifyArgs) -> Result<()> {
    let labels = clean_labels(&args.labels)?;
    let client = ctx.api_client()?;

    let req = IssueLabelRequest { labels };
    client.set_issue_labels(&args.number, &req).await?;
    success!("Issue #{} 标签已替换", args.number);

    Ok(())
}

/// 删除指定标签
async fn run_remove(ctx: &AppContext, args: &LabelRemoveArgs) -> Result<()> {
    let client = ctx.api_client()?;
    client.remove_issue_label(&args.number, &args.name).await?;
    success!("Issue #{} 标签 '{}' 已删除", args.number, args.name);

    Ok(())
}

/// 清空所有标签
async fn run_clear(ctx: &AppContext, args: &LabelClearArgs) -> Result<()> {
    let client = ctx.api_client()?;
    client.clear_issue_labels(&args.number).await?;
    success!("Issue #{} 所有标签已清空", args.number);

    Ok(())
}

/// 清理标签列表：去空白、去空值、去重
fn clean_labels(raw: &[String]) -> Result<Vec<String>> {
    let labels: Vec<String> = raw
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    if labels.is_empty() {
        anyhow::bail!("没有有效的标签");
    }

    Ok(labels)
}
