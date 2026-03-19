//! cnb workspace closed-clean 子命令 - 清理已关闭的工作区

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::concurrent::run_concurrent;
use cnb_tui::{fail, info, success};

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
        if all_workspaces.len() >= usize::try_from(resp.total).unwrap_or(usize::MAX) {
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
    let failed = run_concurrent(&all_workspaces, 10, |ws| {
        let api_client = client;
        let slug = ws.slug.clone();
        let pipeline_id = ws.pipeline_id.clone();
        async move {
            match api_client.delete_workspace(&pipeline_id).await {
                Ok(()) => {
                    success!("已清理工作区 slug={} pipelineId={}", slug, pipeline_id);
                    Ok(())
                }
                Err(e) => {
                    fail!("清理失败 slug={} pipelineId={} err={e}", slug, pipeline_id);
                    Err(e)
                }
            }
        }
    })
    .await;

    if failed > 0 {
        anyhow::bail!("工作区清理完成，但有 {failed} 个删除失败");
    }

    Ok(())
}
