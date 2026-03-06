//! Issue 子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod assigners;
pub mod close;
pub mod comment;
pub mod create;
pub mod download;
pub mod exist;
pub mod edit;
pub mod label;
pub mod list;
pub mod mine;
pub mod reopen;
pub mod view;

/// Issue 管理
#[derive(Debug, Parser)]
pub struct IssueCommand {
    #[command(subcommand)]
    pub subcommand: IssueSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum IssueSubcommand {
    /// 列出仓库的 Issue
    List(list::ListArgs),

    /// 列出与我相关的 Issue
    Mine,

    /// 创建 Issue
    Create(create::CreateArgs),

    /// 关闭 Issue
    Close(close::CloseArgs),

    /// 创建 Issue 评论
    Comment(comment::CommentArgs),

    /// 检查 Issue 是否存在
    Exist(exist::ExistArgs),

    /// 下载 Issue 为 Markdown 文件
    Download(download::DownloadArgs),

    /// Issue 处理人管理（获取/添加）
    Assigners(assigners::AssignersArgs),

    /// 查看 Issue 详情
    View(view::ViewArgs),

    /// 编辑 Issue
    Edit(edit::EditArgs),

    /// 重新打开 Issue
    Reopen(reopen::ReopenArgs),

    /// 标签管理
    Label(label::LabelArgs),
}

impl IssueCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            IssueSubcommand::List(args) => list::run(ctx, args).await,
            IssueSubcommand::Mine => mine::run(ctx).await,
            IssueSubcommand::Create(args) => create::run(ctx, args).await,
            IssueSubcommand::Close(args) => close::run(ctx, args).await,
            IssueSubcommand::Comment(args) => comment::run(ctx, args).await,
            IssueSubcommand::Exist(args) => exist::run(ctx, args).await,
            IssueSubcommand::Download(args) => download::run(ctx, args).await,
            IssueSubcommand::Assigners(args) => assigners::run(ctx, args).await,
            IssueSubcommand::View(args) => view::run(ctx, args).await,
            IssueSubcommand::Edit(args) => edit::run(ctx, args).await,
            IssueSubcommand::Reopen(args) => reopen::run(ctx, args).await,
            IssueSubcommand::Label(args) => label::run(ctx, args).await,
        }
    }
}
