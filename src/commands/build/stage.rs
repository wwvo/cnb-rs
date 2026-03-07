//! cnb build stage 子命令 - 查看 Stage 详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查看 Stage 详情
#[derive(Debug, Parser)]
pub struct StageArgs {
    /// 构建号
    pub sn: String,

    /// 流水线 ID
    pub pipeline_id: String,

    /// Stage ID
    pub stage_id: String,
}

/// 格式化毫秒耗时
fn format_duration_ms(ms: i64) -> String {
    if ms <= 0 {
        return "-".to_string();
    }
    let secs = ms / 1000;
    if secs < 60 {
        format!("{secs}s")
    } else {
        let mins = secs / 60;
        let remaining = secs % 60;
        format!("{mins}m {remaining:02}s")
    }
}

/// 执行 build stage 命令
pub async fn run(ctx: &AppContext, args: &StageArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let result = client.get_build_stage(&args.sn, &args.pipeline_id, &args.stage_id).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    println!("Stage:    {}", result.name);
    println!("状态:     {}", result.status);
    println!("耗时:     {}", format_duration_ms(result.duration));

    if !result.error.is_empty() {
        println!("错误:     {}", result.error);
    }

    if !result.content.is_empty() {
        println!("日志:");
        for line in &result.content {
            println!("  {line}");
        }
    }

    Ok(())
}
