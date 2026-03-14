//! cnb build start 子命令 - 触发构建

use anyhow::Result;
use clap::Parser;
use cnb_api::types::StartBuildRequest;
use cnb_core::context::AppContext;
use cnb_tui::{fail, success};
use std::collections::HashMap;

/// 触发构建
#[derive(Debug, Parser)]
pub struct StartArgs {
    /// 触发分支
    #[arg(short = 'b', long = "branch")]
    pub branch: Option<String>,

    /// 触发 tag（优先级高于 branch）
    #[arg(short = 't', long = "tag")]
    pub tag: Option<String>,

    /// Commit ID（优先级高于 tag）
    #[arg(long = "sha")]
    pub sha: Option<String>,

    /// 事件名（默认 api_trigger）
    #[arg(short = 'e', long = "event")]
    pub event: Option<String>,

    /// 配置文件内容（YAML）
    #[arg(long = "config")]
    pub config: Option<String>,

    /// 环境变量（key=value 格式，可多次指定）
    #[arg(long = "env", value_delimiter = ',')]
    pub env: Vec<String>,

    /// 等待构建正式触发再返回
    #[arg(long = "sync", default_value_t = false)]
    pub sync: bool,
}

/// 执行 build start 命令
pub async fn run(ctx: &AppContext, args: &StartArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let env = if args.env.is_empty() {
        None
    } else {
        let mut map = HashMap::new();
        for item in &args.env {
            if let Some((key, value)) = item.split_once('=') {
                map.insert(key.to_string(), value.to_string());
            }
        }
        Some(map)
    };

    let req = StartBuildRequest {
        branch: args.branch.clone(),
        tag: args.tag.clone(),
        sha: args.sha.clone(),
        event: args.event.clone(),
        config: args.config.clone(),
        env,
        sync: if args.sync {
            Some("true".to_string())
        } else {
            None
        },
    };

    let result = client.start_build(&req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    if result.success {
        success!("构建已触发");
        if !result.sn.is_empty() {
            eprintln!("  构建号: {}", result.sn);
        }
        if !result.build_log_url.is_empty() {
            eprintln!("  日志: {}", result.build_log_url);
        }
    } else {
        fail!("构建触发失败: {}", result.message);
    }

    Ok(())
}
