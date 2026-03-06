//! cnb issue comment 子命令 - 创建 Issue 评论

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateCommentRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 创建 Issue 评论
#[derive(Debug, Parser)]
pub struct CommentArgs {
    /// Issue 编号
    #[arg(short = 'n', long = "number")]
    pub number: String,

    /// 评论内容
    #[arg(short = 'c', long = "comment")]
    pub comment: String,
}

/// 执行 issue comment 命令
pub async fn run(ctx: &AppContext, args: &CommentArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = CreateCommentRequest {
        body: args.comment.clone(),
    };

    client.create_issue_comment(&args.number, &req).await?;
    success!("Issue #{} 评论已创建", args.number);

    Ok(())
}
