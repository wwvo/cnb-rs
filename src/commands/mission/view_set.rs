//! cnb mission view set 子命令 - 设置视图配置

use anyhow::Result;
use clap::Parser;
use cnb_api::types::MissionViewConfig;
use cnb_core::context::AppContext;
use cnb_tui::success;
use std::io::Read;

/// 设置任务集视图配置
#[derive(Debug, Parser)]
pub struct ViewSetArgs {
    /// 任务集路径
    pub mission: String,

    /// 从 JSON 文件读取配置
    #[arg(long = "file")]
    pub file: Option<String>,

    /// 从标准输入读取
    #[arg(long = "stdin", default_value_t = false)]
    pub stdin: bool,
}

/// 执行 mission view set 命令
pub async fn run(ctx: &AppContext, args: &ViewSetArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let json_str = if let Some(ref path) = args.file {
        std::fs::read_to_string(path)?
    } else if args.stdin {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        anyhow::bail!("请指定 --file <path> 或 --stdin");
    };

    let config: MissionViewConfig = serde_json::from_str(&json_str)?;
    client
        .set_mission_view_config(&args.mission, &config)
        .await?;
    success!("视图配置已更新");

    Ok(())
}
