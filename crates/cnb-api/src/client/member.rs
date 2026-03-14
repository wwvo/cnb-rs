//! 成员管理相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::*;
use urlencoding::encode;

impl CnbClient {
    // === 仓库成员 ===

    /// 列出仓库直接成员
    pub async fn list_repo_members(
        &self,
        repo: &str,
        role: Option<&str>,
        search: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<MemberInfo>, ApiError> {
        let repo = Self::encode_path(repo);
        let mut url = format!(
            "{}{repo}/-/members?page={page}&page_size={page_size}",
            self.base_url
        );
        if let Some(r) = role {
            url.push_str(&format!("&role={}", encode(r)));
        }
        if let Some(s) = search {
            url.push_str(&format!("&search={}", encode(s)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 添加仓库成员
    pub async fn add_repo_member(
        &self,
        repo: &str,
        username: &str,
        req: &MemberRequest,
    ) -> Result<(), ApiError> {
        let repo = Self::encode_path(repo);
        let username = encode(username);
        let url = format!("{}{repo}/-/members/{username}", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 更新仓库成员权限
    pub async fn update_repo_member(
        &self,
        repo: &str,
        username: &str,
        req: &MemberRequest,
    ) -> Result<(), ApiError> {
        let repo = Self::encode_path(repo);
        let username = encode(username);
        let url = format!("{}{repo}/-/members/{username}", self.base_url);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 移除仓库成员
    pub async fn remove_repo_member(&self, repo: &str, username: &str) -> Result<(), ApiError> {
        let repo = Self::encode_path(repo);
        let username = encode(username);
        let url = format!("{}{repo}/-/members/{username}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 查看自己在仓库的权限
    pub async fn get_repo_access_level(
        &self,
        repo: &str,
        include_inherit: bool,
    ) -> Result<MemberAccessLevel, ApiError> {
        let repo = Self::encode_path(repo);
        let mut url = format!("{}{repo}/-/members/access-level", self.base_url);
        if include_inherit {
            url.push_str("?include_inherit=true");
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 查看指定成员在仓库的权限层级
    pub async fn get_repo_user_access(
        &self,
        repo: &str,
        username: &str,
    ) -> Result<Vec<MemberAccessLevelInPath>, ApiError> {
        let repo = Self::encode_path(repo);
        let username = encode(username);
        let url = format!("{}{repo}/-/members/{username}/access-level", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 列出仓库继承成员
    pub async fn list_repo_inherited_members(
        &self,
        repo: &str,
        role: Option<&str>,
        search: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<InheritMembersGroup>, ApiError> {
        let repo = Self::encode_path(repo);
        let mut url = format!(
            "{}{repo}/-/inherit-members?page={page}&page_size={page_size}",
            self.base_url
        );
        if let Some(r) = role {
            url.push_str(&format!("&role={}", encode(r)));
        }
        if let Some(s) = search {
            url.push_str(&format!("&search={}", encode(s)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 列出仓库所有有效成员
    pub async fn list_repo_all_members(
        &self,
        slug: &str,
        opts: &ListRepoAllMembersOptions,
    ) -> Result<Vec<MemberInfo>, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/list-members?page={}&page_size={}",
            self.base_url, opts.page, opts.page_size
        );
        if let Some(role) = &opts.role {
            url.push_str(&format!("&role={}", encode(role)));
        }
        if let Some(search) = &opts.search {
            url.push_str(&format!("&search={}", encode(search)));
        }
        if let Some(names) = &opts.names {
            url.push_str(&format!("&names={}", encode(names)));
        }
        if let Some(order_by) = &opts.order_by {
            url.push_str(&format!("&order_by={}", encode(order_by)));
        }
        if opts.desc {
            url.push_str("&desc=true");
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    // === 外部贡献者 ===

    /// 列出外部贡献者
    pub async fn list_outside_collaborators(
        &self,
        slug: &str,
        role: Option<&str>,
        search: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<OutsideCollaborator>, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/outside-collaborators?page={page}&page_size={page_size}",
            self.base_url
        );
        if let Some(r) = role {
            url.push_str(&format!("&role={}", encode(r)));
        }
        if let Some(s) = search {
            url.push_str(&format!("&search={}", encode(s)));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新外部贡献者权限
    pub async fn update_outside_collaborator(
        &self,
        slug: &str,
        username: &str,
        role: &str,
    ) -> Result<(), ApiError> {
        let slug = Self::encode_path(slug);
        let username = encode(username);
        let role = encode(role);
        let url = format!(
            "{}{slug}/-/outside-collaborators/{username}?role={role}",
            self.base_url
        );
        let resp = self.http.put(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 移除外部贡献者
    pub async fn remove_outside_collaborator(
        &self,
        slug: &str,
        username: &str,
    ) -> Result<(), ApiError> {
        let slug = Self::encode_path(slug);
        let username = encode(username);
        let url = format!("{}{slug}/-/outside-collaborators/{username}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }
}
