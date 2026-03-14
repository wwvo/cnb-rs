//! cnb registry tag list 子命令 - 列出制品标签

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListPackageTagsOptions;
use cnb_core::context::AppContext;

/// 列出制品标签
#[derive(Debug, Parser)]
pub struct TagListArgs {
    /// 制品类型（docker/npm/pypi/maven/helm 等）
    pub pkg_type: String,

    /// 制品名称
    pub name: String,

    /// 制品库路径（组织/制品库）
    #[arg(short = 'r', long = "registry")]
    pub registry: String,

    /// 按标签名搜索
    #[arg(long = "tag-name")]
    pub tag_name: Option<String>,

    /// 排序方式
    #[arg(long = "order")]
    pub ordering: Option<String>,
}

/// 执行 registry tag list 命令
pub async fn run(ctx: &AppContext, args: &TagListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let opts = ListPackageTagsOptions {
        tag_name: args.tag_name.clone(),
        ordering: args.ordering.clone(),
        page: 1,
        page_size: 100,
    };
    let tags = client
        .list_package_tags(&args.registry, &args.pkg_type, &args.name, &opts)
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&tags)?);
        return Ok(());
    }

    if tags.is_empty() {
        eprintln!("没有找到标签");
        return Ok(());
    }

    // 标签结构因制品类型而异，统一 JSON 输出
    println!("{}", serde_json::to_string_pretty(&tags)?);

    Ok(())
}
