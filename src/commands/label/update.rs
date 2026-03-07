//! cnb label update 子命令 - 更新标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateLabelRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 更新标签
#[derive(Debug, Parser)]
pub struct UpdateArgs {
    /// 当前标签名称
    pub name: String,

    /// 新名称
    #[arg(long = "new-name")]
    pub new_name: Option<String>,

    /// 新颜色（十六进制，不含 #）
    #[arg(short = 'c', long = "color")]
    pub color: Option<String>,

    /// 新描述
    #[arg(short = 'd', long = "description")]
    pub description: Option<String>,
}

/// 执行 label update 命令
pub async fn run(ctx: &AppContext, args: &UpdateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = UpdateLabelRequest {
        new_name: args.new_name.clone(),
        color: args.color.clone(),
        description: args.description.clone(),
    };

    let label = client.update_label(&args.name, &req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&label)?);
        return Ok(());
    }

    if let Some(ref new_name) = args.new_name {
        success!("标签 {} 已重命名为 {new_name}", args.name);
    } else {
        success!("标签 {} 已更新", label.name);
    }

    Ok(())
}
