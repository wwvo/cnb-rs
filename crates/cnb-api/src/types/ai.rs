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
    /// CNB 扩展字段：`model_response`
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
    /// 1 = 开始思考，3 = 思考结束
    #[serde(rename = "type")]
    pub resp_type: i32,
    pub event_name: Option<String>,
}

impl ChatMessage {
    /// 创建 system 消息
    #[must_use]
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_string(),
        }
    }

    /// 创建 user 消息
    #[must_use]
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_string(),
        }
    }

    /// 创建 assistant 消息
    #[must_use]
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_message_system() {
        let msg = ChatMessage::system("你是助手");
        assert_eq!(msg.content, "你是助手");
        let json = serde_json::to_string(&msg).unwrap_or_else(|e| panic!("序列化失败：{e}"));
        assert!(json.contains(r#""role":"system""#));
    }

    #[test]
    fn chat_message_user() {
        let msg = ChatMessage::user("你好");
        let json = serde_json::to_string(&msg).unwrap_or_else(|e| panic!("序列化失败：{e}"));
        assert!(json.contains(r#""role":"user""#));
    }

    #[test]
    fn chat_message_assistant() {
        let msg = ChatMessage::assistant("回答");
        let json = serde_json::to_string(&msg).unwrap_or_else(|e| panic!("序列化失败：{e}"));
        assert!(json.contains(r#""role":"assistant""#));
    }

    #[test]
    fn role_roundtrip() {
        // 序列化 → 反序列化往返
        let original = Role::System;
        let json = serde_json::to_string(&original).unwrap_or_else(|e| panic!("序列化失败：{e}"));
        assert_eq!(json, r#""system""#);
        let deserialized: Role =
            serde_json::from_str(&json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert!(matches!(deserialized, Role::System));
    }

    #[test]
    fn chat_completions_request_serialize() {
        let req = ChatCompletionsRequest {
            model: "deepseek-r1".to_string(),
            stream: true,
            messages: vec![ChatMessage::system("sys"), ChatMessage::user("hello")],
        };
        let json = serde_json::to_string(&req).unwrap_or_else(|e| panic!("序列化失败：{e}"));
        assert!(json.contains(r#""model":"deepseek-r1""#));
        assert!(json.contains(r#""stream":true"#));
        assert!(json.contains("messages"));
    }

    #[test]
    fn chat_completions_response_deserialize() {
        let json = r#"{
            "id": "chatcmpl-123",
            "model": "deepseek-r1",
            "created": 1700000000,
            "choices": [{
                "index": 0,
                "finish_reason": "stop",
                "message": {"role": "assistant", "content": "你好！"}
            }]
        }"#;
        let resp: ChatCompletionsResponse =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(resp.id, "chatcmpl-123");
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(resp.choices[0].message.content, "你好！");
        assert_eq!(resp.choices[0].finish_reason, Some("stop".to_string()));
    }

    #[test]
    fn chat_stream_chunk_deserialize() {
        let json = r#"{
            "id": "chunk-1",
            "model": "deepseek-r1",
            "created": 1700000000,
            "choices": [{
                "index": 0,
                "finish_reason": null,
                "delta": {"content": "Hello", "reasoning_content": ""}
            }],
            "model_response": null
        }"#;
        let chunk: ChatStreamChunk =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(chunk.choices[0].delta.content, "Hello");
        assert!(chunk.model_response.is_none());
    }

    #[test]
    fn chat_stream_chunk_with_model_response() {
        let json = r#"{
            "id": null,
            "model": null,
            "created": null,
            "choices": [{
                "index": 0,
                "finish_reason": null,
                "delta": {"content": "", "reasoning_content": "思考中..."}
            }],
            "model_response": {"type": 1, "event_name": "thinking_start"}
        }"#;
        let chunk: ChatStreamChunk =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        let mr = chunk
            .model_response
            .as_ref()
            .unwrap_or_else(|| panic!("model_response 不应为 None"));
        assert_eq!(mr.resp_type, 1);
        assert_eq!(chunk.choices[0].delta.reasoning_content, "思考中...");
    }
}
