//! AI Chat API 类型定义

use serde::{Deserialize, Serialize};

/// AI Chat 消息角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// AI Chat 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

/// AI Chat Completions 请求
#[derive(Debug, Serialize)]
pub struct ChatCompletionsRequest {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<ChatMessage>,
}

/// AI Chat Completions 非流式响应
#[derive(Debug, Deserialize)]
pub struct ChatCompletionsResponse {
    pub id: String,
    pub model: String,
    pub created: i64,
    pub choices: Vec<ChatChoice>,
}

/// AI Chat 响应选项
#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub index: i32,
    pub finish_reason: Option<String>,
    pub message: ChatChoiceMessage,
}

/// AI Chat 响应消息
#[derive(Debug, Deserialize)]
pub struct ChatChoiceMessage {
    pub role: String,
    pub content: String,
}

/// SSE 流式响应中的单个 chunk（data: 行解析后的结构）
#[derive(Debug, Deserialize)]
pub struct ChatStreamChunk {
    pub id: Option<String>,
    pub model: Option<String>,
    pub created: Option<i64>,
    pub choices: Vec<ChatStreamChoice>,
    /// CNB 扩展字段：model_response
    pub model_response: Option<ModelResponse>,
}

/// SSE 流式响应选项
#[derive(Debug, Deserialize)]
pub struct ChatStreamChoice {
    pub index: i32,
    pub finish_reason: Option<String>,
    pub delta: ChatStreamDelta,
}

/// SSE 流式响应增量内容
#[derive(Debug, Deserialize)]
pub struct ChatStreamDelta {
    #[serde(default)]
    pub content: String,
    /// 推理/思考过程内容
    #[serde(default)]
    pub reasoning_content: String,
}

/// CNB 扩展：模型响应状态
#[derive(Debug, Deserialize)]
pub struct ModelResponse {
    /// 1 = 开始思考, 3 = 思考结束
    #[serde(rename = "type")]
    pub resp_type: i32,
    pub event_name: Option<String>,
}

impl ChatMessage {
    /// 创建 system 消息
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_string(),
        }
    }

    /// 创建 user 消息
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_string(),
        }
    }

    /// 创建 assistant 消息
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
        }
    }
}
