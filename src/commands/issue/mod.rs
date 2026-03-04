//! Issue 子命令组

use clap::Parser;

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
}
