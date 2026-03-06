//! cnb group quota 子命令 - 查看组织特权额度

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::table::{Column, Table};

/// 查看组织特权额度
#[derive(Debug, Parser)]
pub struct QuotaArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,
}

pub async fn run(ctx: &AppContext, args: &QuotaArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let amount = client.get_special_amount(&args.group).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&amount)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("资源类型", 20),
        Column::new("额度", 16),
        Column::new("说明", 20),
        Column::new("过期时间", 20),
    ]);

    table.add_row(vec![
        "构建算力".to_string(),
        format!("{:.2} 核时", amount.compute_build_corehour),
        amount.compute_build_desc.clone(),
        amount.compute_build_expire.as_deref().unwrap_or_default().to_string(),
    ]);
    table.add_row(vec![
        "构建 GPU 算力".to_string(),
        format!("{:.2} 核时", amount.compute_build_gpu_corehour),
        amount.compute_build_gpu_desc.clone(),
        amount.compute_build_gpu_expire.as_deref().unwrap_or_default().to_string(),
    ]);
    table.add_row(vec![
        "开发算力".to_string(),
        format!("{:.2} 核时", amount.compute_develop_corehour),
        amount.compute_develop_desc.clone(),
        amount.compute_develop_expire.as_deref().unwrap_or_default().to_string(),
    ]);
    table.add_row(vec![
        "开发 GPU 算力".to_string(),
        format!("{:.2} 核时", amount.compute_develop_gpu_corehour),
        amount.compute_develop_gpu_desc.clone(),
        amount.compute_develop_gpu_expire.as_deref().unwrap_or_default().to_string(),
    ]);
    table.add_row(vec![
        "Git 存储".to_string(),
        format!("{:.2} GiB", amount.storage_git_gib),
        amount.storage_git_desc.clone(),
        amount.storage_git_expire.as_deref().unwrap_or_default().to_string(),
    ]);
    table.add_row(vec![
        "对象存储".to_string(),
        format!("{:.2} GiB", amount.storage_object_gib),
        amount.storage_object_desc.clone(),
        amount.storage_object_expire.as_deref().unwrap_or_default().to_string(),
    ]);

    table.print();

    Ok(())
}
