//! CNB API 错误类型定义

/// API 调用错误
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// HTTP 请求返回非成功状态码
    #[error("API 请求失败 (HTTP {status}): {body}")]
    HttpStatus { status: u16, body: String },

    /// 认证失败（Token 无效或缺失）
    #[error("认证失败：{0}")]
    Auth(String),

    /// 网络错误
    #[error("网络错误：{0}")]
    Network(#[from] reqwest::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON 解析错误：{0}")]
    Json(#[from] serde_json::Error),

    /// URL 解析错误
    #[error("URL 解析错误：{0}")]
    Url(#[from] url::ParseError),

    /// 资源未找到
    #[error("{0}")]
    NotFound(String),

    /// 通用 API 错误
    #[error("{0}")]
    Api(String),
}
