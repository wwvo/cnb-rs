//! cnb mission view get 子命令 - 获取视图配置

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 获取任务集视图配置
#[derive(Debug, Parser)]
pub struct ViewGetArgs {
    /// 任务集路径
    pub mission: String,

    /// 视图 ID
    #[arg(long = "id")]
    pub view_id: String,
}

/// 执行 mission view get 命令
pub async fn run(ctx: &AppContext, args: &ViewGetArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let config = client
        .get_mission_view_config(&args.mission, &args.view_id)
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&config)?);
        return Ok(());
    }

    let view_type = if let Some(s) = config.view_type.as_str() {
        s.to_string()
    } else {
        config.view_type.to_string()
    };

    println!("View ID:  {}", config.id);
    println!("Type:     {view_type}");
    println!("Fields:   {}", config.fields.len());
    println!("Filters:  {}", config.selectors.len());
    println!("Sorts:    {}", config.sorts.len());

    if !config.group.is_null()
        && let Some(field) = config.group.get("field").and_then(|value| value.as_str())
    {
        println!("Group By: {field}");
    }

    Ok(())
}
