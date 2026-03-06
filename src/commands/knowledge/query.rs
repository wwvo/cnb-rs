//! cnb knowledge query 子命令

use anyhow::Result;
use clap::Parser;
use cnb_api::types::QueryKnowledgeBaseRequest;
use cnb_core::context::AppContext;

/// 查询知识库参数
#[derive(Debug, Parser)]
pub struct QueryArgs {
    /// 查询文本
    #[arg(long = "query", short = 'q')]
    pub query: String,

    /// 分数阈值
    #[arg(long = "score-threshold")]
    pub score_threshold: Option<f64>,

    /// 返回结果数量上限
    #[arg(long = "top-k")]
    pub top_k: Option<i32>,
}

/// 查询知识库
pub async fn run(ctx: &AppContext, args: &QueryArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = QueryKnowledgeBaseRequest {
        query: args.query.clone(),
        score_threshold: args.score_threshold,
        top_k: args.top_k,
    };

    let results = client.query_knowledge_base(&req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    for r in &results {
        let url = r.metadata.get("url").map_or("-", String::as_str);
        let file_type = r.metadata.get("type").map_or("-", String::as_str);
        println!("{};{:.4};{}", url, r.score, file_type);
    }

    Ok(())
}
