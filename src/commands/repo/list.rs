//! cnb repo list 子命令 — 列出仓库

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListReposOptions;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 列出仓库
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 用户名或组织名（不指定则列出当前用户的仓库）
    pub owner: Option<String>,

    /// 最大列出数量
    #[arg(short = 'L', long, default_value_t = 30)]
    pub limit: u32,

    /// 按可见性过滤（public/private/secret）
    #[arg(long)]
    pub visibility: Option<String>,

    /// 排序字段（created_at/last_updated_at/stars）
    #[arg(long)]
    pub sort: Option<String>,

    /// 倒序排列
    #[arg(long)]
    pub desc: bool,

    /// 按关键词搜索
    #[arg(short, long)]
    pub search: Option<String>,
}

pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let opts = ListReposOptions {
        page: 1,
        page_size: args.limit.min(100),
        search: args.search.clone(),
        filter_type: args.visibility.clone(),
        order_by: args.sort.clone(),
        desc: args.desc,
    };

    let repos = match &args.owner {
        None => client.list_my_repos(&opts).await?,
        Some(owner) => {
            // 尝试作为组织查询，如果 404 则作为用户查询
            match client.list_group_repos(owner, &opts).await {
                Ok(repos) => repos,
                Err(cnb_api::error::ApiError::NotFound(_)) => {
                    client.list_user_repos(owner, &opts).await?
                }
                Err(e) => return Err(e.into()),
            }
        }
    };

    if repos.is_empty() {
        info!("没有找到仓库");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&repos)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("仓库", 35),
        Column::new("描述", 30),
        Column::new("⭐", 6),
        Column::new("语言", 10),
        Column::new("更新时间", 20),
    ]);

    for repo in &repos {
        let language = repo
            .languages
            .as_ref()
            .map_or("-".to_string(), |l| {
                if l.language.is_empty() { "-".to_string() } else { l.language.clone() }
            });

        let updated_at = repo
            .last_updated_at
            .as_ref()
            .and_then(|t| if t.valid { Some(t.time.clone()) } else { None })
            .unwrap_or_else(|| "-".to_string());

        // 截断更新时间，只保留日期部分
        let updated_date = if updated_at.len() >= 10 {
            updated_at[..10].to_string()
        } else {
            updated_at
        };

        let desc = if repo.description.len() > 28 {
            format!("{}...", &repo.description[..28])
        } else {
            repo.description.clone()
        };

        table.add_row(vec![
            repo.path.clone(),
            desc,
            repo.star_count.to_string(),
            language,
            updated_date,
        ]);
    }

    table.print();

    Ok(())
}
