use super::CnbClient;
use crate::error::ApiError;
use crate::types::StarUsers;

impl CnbClient {
    /// 获取全部 Star 用户（自动分页）
    ///
    /// Star API 返回 `{ total, users }` 包装结构，无法复用通用 `paginate()`，
    /// 因此使用独立的分页循环。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if any paginated request fails, a response cannot be
    /// deserialized, or the CNB API returns a non-success status.
    pub async fn list_star_users(&self) -> Result<StarUsers, ApiError> {
        const PAGE_SIZE_U32: u32 = 100;
        const PAGE_SIZE_USIZE: usize = 100;
        let mut all_users = Vec::new();
        let mut page = 0u32;
        loop {
            let url = format!(
                "{}{}/-/star-users?filter_type=all&page={page}&page_size={PAGE_SIZE_U32}",
                self.base_url, self.repo
            );
            let resp = self.http.get(&url).send().await?;
            let result: StarUsers = Self::handle_response(resp).await?;
            let total = result.total;
            let count = result.users.len();
            all_users.extend(result.users);
            if count < PAGE_SIZE_USIZE {
                return Ok(StarUsers {
                    total,
                    users: all_users,
                });
            }
            page += 1;
        }
    }
}
