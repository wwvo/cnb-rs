//! Knowledge 知识库子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod clean;
pub mod info;
pub mod list_models;
pub mod query;

/// 知识库管理
#[derive(Debug, Parser)]
pub struct KnowledgeCommand {
    #[command(subcommand)]
    pub subcommand: KnowledgeSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum KnowledgeSubcommand {
    /// 列出支持的 Embedding 模型
    #[command(name = "list-models")]
    ListModels,
    /// 获取知识库信息
    Info,
    /// 清除知识库
    Clean,
    /// 查询知识库
    Query(query::QueryArgs),
}

impl KnowledgeCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            KnowledgeSubcommand::ListModels => list_models::run(ctx).await,
            KnowledgeSubcommand::Info => info::run(ctx).await,
            KnowledgeSubcommand::Clean => clean::run(ctx).await,
            KnowledgeSubcommand::Query(args) => query::run(ctx, args).await,
        }
    }
}
