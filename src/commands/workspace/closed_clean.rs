//! cnb workspace closed-clean 子命令 - 清理已关闭的工作区

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::{info, success, fail};
use futures::stream::{self, StreamExt};

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
        info!("没有已关闭的云原生工作区需要清理");
        return Ok(());
    }

    info!("共找到 {} 个已关闭的工作区\n", all_workspaces.len());

    // 并发清理（最多 10 并发）
    stream::iter(all_workspaces.iter())
        .for_each_concurrent(10, |ws| {
            let client = &client;
            async move {
                match client.delete_workspace(&ws.pipeline_id).await {
                    Ok(()) => {
                        success!("已清理工作区 slug={} pipelineId={}", ws.slug, ws.pipeline_id);
                    }
                    Err(e) => {
                        fail!("清理失败 slug={} pipelineId={} err={e}", ws.slug, ws.pipeline_id);
                    }
                }
            }
        })
        .await;

    Ok(())
}
