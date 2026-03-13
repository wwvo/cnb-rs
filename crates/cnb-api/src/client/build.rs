//! 构建相关 API

use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;
use urlencoding::encode;

impl CnbClient {
    /// 触发构建
    pub async fn start_build(&self, req: &StartBuildRequest) -> Result<BuildResult, ApiError> {
        let url = format!("{}{}/-/build/start", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 停止构建
    pub async fn stop_build(&self, sn: &str) -> Result<BuildResult, ApiError> {
        let sn = encode(sn);
        let url = format!("{}{}/-/build/stop/{sn}", self.base_url, self.repo);
        let resp = self.http.post(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 查询构建状态
    pub async fn get_build_status(&self, sn: &str) -> Result<BuildStatusResult, ApiError> {
        let sn = encode(sn);
        let url = format!("{}{}/-/build/status/{sn}", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出构建记录
    pub async fn get_build_logs(&self, opts: &BuildListOptions) -> Result<BuildLogsResult, ApiError> {
        let mut url = format!("{}{}/-/build/logs?page={}&page_size={}",
            self.base_url, self.repo, opts.page, opts.page_size);
        if let Some(ref status) = opts.status {
            url.push_str(&format!("&status={}", encode(status)));
        }
        if let Some(ref event) = opts.event {
            url.push_str(&format!("&event={}", encode(event)));
        }
        if let Some(ref source_ref) = opts.source_ref {
            url.push_str(&format!("&sourceRef={}", encode(source_ref)));
        }
        if let Some(ref user_name) = opts.user_name {
            url.push_str(&format!("&userName={}", encode(user_name)));
        }
        if let Some(ref create_time) = opts.create_time {
            url.push_str(&format!("&createTime={}", encode(create_time)));
        }
        if let Some(ref end_time) = opts.end_time {
            url.push_str(&format!("&endTime={}", encode(end_time)));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 查看 Stage 详情
    pub async fn get_build_stage(&self, sn: &str, pipeline_id: &str, stage_id: &str) -> Result<BuildStageResult, ApiError> {
        let sn = encode(sn);
        let pipeline_id = encode(pipeline_id);
        let stage_id = encode(stage_id);
        let url = format!("{}{}/-/build/logs/stage/{sn}/{pipeline_id}/{stage_id}", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 下载 Runner 日志
    pub async fn download_build_log(&self, pipeline_id: &str) -> Result<String, ApiError> {
        let pipeline_id = encode(pipeline_id);
        let url = format!("{}{}/-/build/runner/download/log/{pipeline_id}", self.base_url, self.repo);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            let text = resp.text().await.map_err(ApiError::Network)?;
            return Ok(text);
        }
        Err(Self::map_error_status(status, resp).await)
    }

    /// 删除构建日志
    pub async fn delete_build_log(&self, sn: &str) -> Result<BuildCommonResult, ApiError> {
        let sn = encode(sn);
        let url = format!("{}{}/-/build/logs/{sn}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 同步定时任务
    pub async fn build_crontab_sync(&self, branch: &str) -> Result<BuildCommonResult, ApiError> {
        let branch = encode(branch);
        let url = format!("{}{}/-/build/crontab/sync/{branch}", self.base_url, self.repo);
        let resp = self.http.post(&url).send().await?;
        Self::handle_response(resp).await
    }
}
