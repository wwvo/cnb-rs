use super::CnbClient;
use crate::error::ApiError;
use crate::types::*;

impl CnbClient {
    /// 获取全部 Star 用户（自动分页）
    ///
    /// Star API 返回 `{ total, users }` 包装结构，无法复用通用 `paginate()`，
    /// 因此使用独立的分页循环。
    pub async fn list_star_users(&self) -> Result<StarUsers, ApiError> {
        let page_size = 100u32;
        let mut all_users = Vec::new();
        let mut page = 0u32;
        loop {
            let url = format!(
                "{}{}/-/star-users?filter_type=all&page={page}&page_size={page_size}",
                self.base_url, self.repo
            );
            let resp = self.http.get(&url).send().await?;
            let result: StarUsers = Self::handle_response(resp).await?;
            let total = result.total;
            let count = result.users.len();
            all_users.extend(result.users);
            if (count as u32) < page_size {
                return Ok(StarUsers {
                    total,
                    users: all_users,
                });
            }
            page += 1;
        }
    }
}
