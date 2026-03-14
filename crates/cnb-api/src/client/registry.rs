//! 制品库管理相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::*;
use urlencoding::encode;

impl CnbClient {
    /// 列出组织下的制品库
    pub async fn list_registries(
        &self,
        slug: &str,
        opts: &ListRegistriesOptions,
    ) -> Result<Vec<Registry>, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/registries?page={}&page_size={}",
            self.base_url, opts.page, opts.page_size
        );
        if let Some(registry_type) = &opts.registry_type {
            url.push_str(&format!("&registry_type={}", encode(registry_type)));
        }
        if let Some(search) = &opts.search {
            url.push_str(&format!("&search={}", encode(search)));
        }
        if let Some(order_by) = &opts.order_by {
            url.push_str(&format!("&order_by={}", encode(order_by)));
        }
        if opts.desc {
            url.push_str("&desc=true");
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 删除制品库
    pub async fn delete_registry(&self, registry: &str) -> Result<(), ApiError> {
        let registry = Self::encode_path(registry);
        let url = format!("{}{registry}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 设置制品库可见性
    pub async fn set_registry_visibility(
        &self,
        registry: &str,
        req: &SetRegistryVisibilityRequest,
    ) -> Result<(), ApiError> {
        let registry = Self::encode_path(registry);
        let url = format!("{}{registry}/-/settings/set_visibility", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出制品
    pub async fn list_packages(
        &self,
        slug: &str,
        pkg_type: &str,
        name: Option<&str>,
        ordering: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Package>, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/packages?type={}&page={page}&page_size={page_size}",
            self.base_url,
            encode(pkg_type),
        );
        if let Some(n) = name {
            url.push_str(&format!("&name={}", encode(n)));
        }
        if let Some(o) = ordering {
            url.push_str(&format!("&ordering={}", encode(o)));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取制品详情
    pub async fn get_package(
        &self,
        slug: &str,
        pkg_type: &str,
        name: &str,
    ) -> Result<PackageDetail, ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!(
            "{}{slug}/-/packages/{}/{}",
            self.base_url,
            encode(pkg_type),
            encode(name),
        );
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 删除制品
    pub async fn delete_package(
        &self,
        slug: &str,
        pkg_type: &str,
        name: &str,
    ) -> Result<(), ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!(
            "{}{slug}/-/packages/{}/{}",
            self.base_url,
            encode(pkg_type),
            encode(name),
        );
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    /// 列出制品标签
    pub async fn list_package_tags(
        &self,
        slug: &str,
        pkg_type: &str,
        name: &str,
        opts: &ListPackageTagsOptions,
    ) -> Result<Vec<PackageTag>, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/packages/{}/{}/-/tags?page={}&page_size={}",
            self.base_url,
            encode(pkg_type),
            encode(name),
            opts.page,
            opts.page_size
        );
        if let Some(tag_name) = &opts.tag_name {
            url.push_str(&format!("&tag_name={}", encode(tag_name)));
        }
        if let Some(ordering) = &opts.ordering {
            url.push_str(&format!("&ordering={}", encode(ordering)));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 获取标签详情
    pub async fn get_package_tag_detail(
        &self,
        slug: &str,
        pkg_type: &str,
        name: &str,
        tag: &str,
        arch: Option<&str>,
    ) -> Result<PackageTag, ApiError> {
        let slug = Self::encode_path(slug);
        let mut url = format!(
            "{}{slug}/-/packages/{}/{}/-/tag/{}",
            self.base_url,
            encode(pkg_type),
            encode(name),
            encode(tag),
        );
        if let Some(a) = arch {
            url.push_str(&format!("?arch={}", encode(a)));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 删除制品标签
    pub async fn delete_package_tag(
        &self,
        slug: &str,
        pkg_type: &str,
        name: &str,
        tag: &str,
    ) -> Result<(), ApiError> {
        let slug = Self::encode_path(slug);
        let url = format!(
            "{}{slug}/-/packages/{}/{}/-/tag/{}",
            self.base_url,
            encode(pkg_type),
            encode(name),
            encode(tag),
        );
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }
}
