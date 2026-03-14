//! 任务集管理相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::*;
use urlencoding::encode;

impl CnbClient {
    /// 列出组织下的任务集
    pub async fn list_missions(
        &self,
        slug: &str,
        search: Option<&str>,
        order_by: Option<&str>,
        desc: bool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Mission>, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/missions?page={page}&page_size={page_size}",
            self.base_url
        );
        if let Some(s) = search {
            url.push_str(&format!("&search={}", encode(s)));
        }
        if let Some(o) = order_by {
            url.push_str(&format!("&order_by={}", encode(o)));
        }
        if desc {
            url.push_str("&desc=true");
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 创建任务集
    pub async fn create_mission(
        &self,
        slug: &str,
        req: &CreateMissionRequest,
    ) -> Result<(), ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!("{}{slug}/-/missions", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除任务集
    pub async fn delete_mission(&self, mission: &str) -> Result<(), ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!("{}{mission}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置任务集可见性
    pub async fn set_mission_visibility(
        &self,
        mission: &str,
        req: &SetMissionVisibilityRequest,
    ) -> Result<(), ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!("{}{mission}/-/settings/set_visibility", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出视图
    pub async fn list_mission_views(&self, mission: &str) -> Result<Vec<MissionView>, ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!("{}{mission}/-/mission/view-list", self.base_url);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取视图配置
    pub async fn get_mission_view_config(
        &self,
        mission: &str,
        view_id: &str,
    ) -> Result<MissionViewConfig, ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!(
            "{}{mission}/-/mission/view?id={}",
            self.base_url,
            encode(view_id)
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 设置视图配置
    pub async fn set_mission_view_config(
        &self,
        mission: &str,
        config: &MissionViewConfig,
    ) -> Result<(), ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!("{}{mission}/-/mission/view", self.base_url);
        let resp = self.http.post(&url).json(config).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 添加/修改视图
    pub async fn put_mission_view(
        &self,
        mission: &str,
        views: &[MissionView],
    ) -> Result<(), ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!("{}{mission}/-/mission/view-list", self.base_url);
        let resp = self.http.put(&url).json(views).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 排序视图
    pub async fn sort_mission_views(
        &self,
        mission: &str,
        views: &[MissionView],
    ) -> Result<(), ApiError> {
        let mission = Self::encode_path(mission);
        let url = format!("{}{mission}/-/mission/view-list", self.base_url);
        let resp = self.http.post(&url).json(views).send().await?;
        Self::handle_empty_response(resp).await
    }
}
