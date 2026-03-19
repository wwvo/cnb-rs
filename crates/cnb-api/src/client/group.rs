use super::CnbClient;
use crate::error::ApiError;
use crate::types::{
    CreateGroupRequest, Group, GroupMember, GroupMemberRequest, GroupSetting, InheritMembersGroup,
    ListGroupMembersOptions, ListGroupsOptions, ListSubGroupsOptions, MemberAccessLevel,
    MemberAccessLevelInPath, SpecialAmount, TransferGroupRequest, UpdateGroupRequest,
    UpdateGroupSettingRequest, UploadLogoRequest, UploadLogoResponse,
};

impl CnbClient {
    // ==================== Logo 上传 ====================

    /// 获取组织 Logo 上传信息。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn upload_logo_info(
        &self,
        group_name: &str,
        req: &UploadLogoRequest,
    ) -> Result<UploadLogoResponse, ApiError> {
        let group_name = Self::encode_path(group_name);
        let url = format!("{}{}/-/upload/logos", self.base_url, group_name);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 将文件内容上传到 COS。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the upload request fails, the COS endpoint returns a
    /// non-success status, or the response body cannot be handled as expected.
    pub async fn post_to_cos(
        &self,
        upload_url: &str,
        form: &std::collections::HashMap<String, String>,
        file_data: Vec<u8>,
    ) -> Result<(), ApiError> {
        let mut multipart = reqwest::multipart::Form::new();
        for (key, value) in form {
            multipart = multipart.text(key.clone(), value.clone());
        }
        let part = reqwest::multipart::Part::bytes(file_data).file_name("file.dat");
        multipart = multipart.part("file", part);

        let resp = self
            .http_plain
            .post(upload_url)
            .multipart(multipart)
            .send()
            .await?;

        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(());
        }
        let text = resp.text().await.unwrap_or_default();
        Err(ApiError::Api(format!("上传失败 HTTP {status}: {text}")))
    }

    // ==================== 组织 CRUD ====================

    /// 列出当前用户的顶层组织
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_top_groups(&self, opts: &ListGroupsOptions) -> Result<Vec<Group>, ApiError> {
        let url = format!("{}user/groups?{}", self.base_url, opts.query_string());
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出用户在指定组织下有权限的子组织
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_groups_under(
        &self,
        slug: &str,
        opts: &ListGroupsOptions,
    ) -> Result<Vec<Group>, ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!(
            "{}user/groups/{}?{}",
            self.base_url,
            slug,
            opts.query_string()
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取组织详情
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_group(&self, group: &str) -> Result<Group, ApiError> {
        let group = Self::encode_path(group);
        let url = format!("{}{}", self.base_url, group);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 创建组织
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn create_group(&self, req: &CreateGroupRequest) -> Result<(), ApiError> {
        let url = format!("{}groups", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 更新组织信息
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn update_group(
        &self,
        group: &str,
        req: &UpdateGroupRequest,
    ) -> Result<(), ApiError> {
        let group = Self::encode_path(group);
        let url = format!("{}{}", self.base_url, group);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 删除组织
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn delete_group(&self, group: &str) -> Result<(), ApiError> {
        let group = Self::encode_path(group);
        let url = format!("{}{}", self.base_url, group);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 转移组织
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn transfer_group(
        &self,
        group: &str,
        req: &TransferGroupRequest,
    ) -> Result<(), ApiError> {
        let group = Self::encode_path(group);
        let url = format!("{}{}/-/transfer", self.base_url, group);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出子组织
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_subgroups(
        &self,
        slug: &str,
        opts: &ListSubGroupsOptions,
    ) -> Result<Vec<Group>, ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!(
            "{}{}/-/sub-groups?{}",
            self.base_url,
            slug,
            opts.query_string()
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    // ==================== 组织配置 ====================

    /// 获取组织配置
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_group_setting(&self, slug: &str) -> Result<GroupSetting, ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!("{}{}/-/settings", self.base_url, slug);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 更新组织配置
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn update_group_setting(
        &self,
        slug: &str,
        req: &UpdateGroupSettingRequest,
    ) -> Result<(), ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!("{}{}/-/settings", self.base_url, slug);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    // ==================== 特权额度 ====================

    /// 获取组织特权额度
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_special_amount(&self, slug: &str) -> Result<SpecialAmount, ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!("{}{}/-/charge/special-amount", self.base_url, slug);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    // ==================== 成员管理 ====================

    /// 列出组织直接成员
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_group_members(
        &self,
        group: &str,
        opts: &ListGroupMembersOptions,
    ) -> Result<Vec<GroupMember>, ApiError> {
        let group = Self::encode_path(group);
        let url = format!(
            "{}{}/-/members?{}",
            self.base_url,
            group,
            opts.query_string()
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出组织继承成员
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_inherit_members(
        &self,
        group: &str,
        opts: &ListGroupMembersOptions,
    ) -> Result<Vec<InheritMembersGroup>, ApiError> {
        let group = Self::encode_path(group);
        let url = format!(
            "{}{}/-/inherit-members?{}",
            self.base_url,
            group,
            opts.query_string()
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 添加组织成员
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn add_group_member(
        &self,
        group: &str,
        username: &str,
        req: &GroupMemberRequest,
    ) -> Result<(), ApiError> {
        let group = Self::encode_path(group);
        let url = format!(
            "{}{}/-/members/{}",
            self.base_url,
            group,
            urlencoding::encode(username)
        );
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 更新组织成员权限
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn update_group_member(
        &self,
        group: &str,
        username: &str,
        req: &GroupMemberRequest,
    ) -> Result<(), ApiError> {
        let group = Self::encode_path(group);
        let url = format!(
            "{}{}/-/members/{}",
            self.base_url,
            group,
            urlencoding::encode(username)
        );
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 移除组织成员
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails or the CNB API returns a non-success
    /// status.
    pub async fn remove_group_member(&self, group: &str, username: &str) -> Result<(), ApiError> {
        let group = Self::encode_path(group);
        let url = format!(
            "{}{}/-/members/{}",
            self.base_url,
            group,
            urlencoding::encode(username)
        );
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 获取当前用户在组织的权限
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_member_access_level(
        &self,
        group: &str,
    ) -> Result<MemberAccessLevel, ApiError> {
        let group = Self::encode_path(group);
        let url = format!("{}{}/-/members/access-level", self.base_url, group);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取指定用户在组织的权限层级
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_member_access_level(
        &self,
        group: &str,
        username: &str,
    ) -> Result<Vec<MemberAccessLevelInPath>, ApiError> {
        let group = Self::encode_path(group);
        let url = format!(
            "{}{}/-/members/{}/access-level",
            self.base_url,
            group,
            urlencoding::encode(username)
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }
}
