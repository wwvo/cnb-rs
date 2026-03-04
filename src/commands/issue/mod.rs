//! Issue 子命令组

use clap::Parser;

pub mod assigners;
pub mod close;
pub mod comment;
pub mod create;
pub mod download;
pub mod exist;
pub mod list;
pub mod mine;

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
}
