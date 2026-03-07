//! cnb label create 子命令 - 创建标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateLabelRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 创建标签
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// 标签名称
    #[arg(short = 'n', long = "name")]
    pub name: String,

    /// 颜色（十六进制，不含 #）
    #[arg(short = 'c', long = "color")]
    pub color: String,

    /// 标签描述
    #[arg(short = 'd', long = "description", default_value = "")]
    pub description: String,
}

/// 执行 label create 命令
pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = CreateLabelRequest {
        name: args.name.clone(),
        color: args.color.clone(),
        description: args.description.clone(),
    };

    let label = client.create_label(&req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&label)?);
        return Ok(());
    }

    success!("标签 {} 已创建", label.name);

    Ok(())
}
