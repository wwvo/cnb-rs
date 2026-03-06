//! cnb repo create 子命令 — 创建仓库

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateRepoRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 创建新仓库
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// 仓库名称
    pub name: String,

    /// 所属组织（不指定则创建到个人空间）
    #[arg(short, long)]
    pub group: Option<String>,

    /// 仓库描述
    #[arg(short, long)]
    pub description: Option<String>,

    /// 开源许可证
    #[arg(short, long)]
    pub license: Option<String>,

    /// 创建私有仓库
    #[arg(long)]
    pub private: bool,

    /// 创建加密仓库
    #[arg(long)]
    pub secret: bool,
}

pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    // 确定组织路径：优先使用 --group，否则使用当前用户名
    let slug = match &args.group {
        Some(g) => g.clone(),
        None => {
            let user = client.me().await?;
            user.username
        }
    };

    let visibility = if args.secret {
        Some("secret".to_string())
    } else if args.private {
        Some("private".to_string())
    } else {
        Some("public".to_string())
    };

    let req = CreateRepoRequest {
        name: args.name.clone(),
        description: args.description.clone(),
        license: args.license.clone(),
        visibility,
    };

    client.create_repo(&slug, &req).await?;

    let domain = ctx.domain();
    success!("仓库已创建：https://{domain}/{slug}/{}", args.name);

    Ok(())
}
