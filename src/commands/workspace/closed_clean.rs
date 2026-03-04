//! cnb workspace closed-clean 子命令 - 清理已关闭的工作区

use anyhow::Result;
use cnb_core::context::AppContext;

const PAGE_SIZE: i32 = 100;

/// 清理所有已关闭的云原生工作区
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;

    // 分页获取所有已关闭的工作区
    let mut all_workspaces = Vec::new();
    let mut page = 1;

    loop {
        let resp = client.list_workspaces("closed", page, PAGE_SIZE).await?;
        if resp.list.is_empty() {
            break;
        }
        all_workspaces.extend(resp.list);
        if (all_workspaces.len() as i64) >= resp.total {
            break;
        }
        page += 1;
    }

    if all_workspaces.is_empty() {
        println!("没有已关闭的云原生工作区需要清理");
        return Ok(());
    }

    println!("共找到 {} 个已关闭的工作区\n", all_workspaces.len());

    for ws in &all_workspaces {
        println!(
            "[INFO] 开始清理工作区 slug={} pipelineId={}",
            ws.slug, ws.pipeline_id
        );

        match client.delete_workspace(&ws.pipeline_id).await {
            Ok(()) => {
                println!(
                    "[SUCCESS] 已清理工作区 slug={} pipelineId={}",
                    ws.slug, ws.pipeline_id
                );
            }
            Err(e) => {
                println!(
                    "[WARN] 清理失败 slug={} pipelineId={} err={e}",
                    ws.slug, ws.pipeline_id
                );
            }
        }
    }

    Ok(())
}
