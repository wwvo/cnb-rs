//! Label 标签管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod create;
pub mod delete;
pub mod issue_add;
pub mod issue_clear;
pub mod issue_list;
pub mod issue_remove;
pub mod issue_set;
pub mod list;
pub mod pull_add;
pub mod pull_clear;
pub mod pull_list;
pub mod pull_remove;
pub mod pull_set;
pub mod update;

/// 标签管理
#[derive(Debug, Parser)]
pub struct LabelCommand {
    #[command(subcommand)]
    pub subcommand: LabelSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum LabelSubcommand {
    /// 列出仓库所有标签
    List(list::ListArgs),
    /// 创建标签
    Create(create::CreateArgs),
    /// 更新标签
    Update(update::UpdateArgs),
    /// 删除标签
    Delete(delete::DeleteArgs),
    /// 列出 Issue 标签
    #[command(name = "issue-list")]
    IssueList(issue_list::IssueListArgs),
    /// 为 Issue 添加标签
    #[command(name = "issue-add")]
    IssueAdd(issue_add::IssueAddArgs),
    /// 替换 Issue 标签
    #[command(name = "issue-set")]
    IssueSet(issue_set::IssueSetArgs),
    /// 移除 Issue 单个标签
    #[command(name = "issue-remove")]
    IssueRemove(issue_remove::IssueRemoveArgs),
    /// 清空 Issue 所有标签
    #[command(name = "issue-clear")]
    IssueClear(issue_clear::IssueClearArgs),
    /// 列出 Pull 标签
    #[command(name = "pull-list")]
    PullList(pull_list::PullListArgs),
    /// 为 Pull 添加标签
    #[command(name = "pull-add")]
    PullAdd(pull_add::PullAddArgs),
    /// 替换 Pull 标签
    #[command(name = "pull-set")]
    PullSet(pull_set::PullSetArgs),
    /// 移除 Pull 单个标签
    #[command(name = "pull-remove")]
    PullRemove(pull_remove::PullRemoveArgs),
    /// 清空 Pull 所有标签
    #[command(name = "pull-clear")]
    PullClear(pull_clear::PullClearArgs),
}

impl LabelCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            LabelSubcommand::List(args) => list::run(ctx, args).await,
            LabelSubcommand::Create(args) => create::run(ctx, args).await,
            LabelSubcommand::Update(args) => update::run(ctx, args).await,
            LabelSubcommand::Delete(args) => delete::run(ctx, args).await,
            LabelSubcommand::IssueList(args) => issue_list::run(ctx, args).await,
            LabelSubcommand::IssueAdd(args) => issue_add::run(ctx, args).await,
            LabelSubcommand::IssueSet(args) => issue_set::run(ctx, args).await,
            LabelSubcommand::IssueRemove(args) => issue_remove::run(ctx, args).await,
            LabelSubcommand::IssueClear(args) => issue_clear::run(ctx, args).await,
            LabelSubcommand::PullList(args) => pull_list::run(ctx, args).await,
            LabelSubcommand::PullAdd(args) => pull_add::run(ctx, args).await,
            LabelSubcommand::PullSet(args) => pull_set::run(ctx, args).await,
            LabelSubcommand::PullRemove(args) => pull_remove::run(ctx, args).await,
            LabelSubcommand::PullClear(args) => pull_clear::run(ctx, args).await,
        }
    }
}
