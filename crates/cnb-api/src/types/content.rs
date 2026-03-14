//! 仓库文件内容相关类型定义

use serde::Deserialize;
use std::fmt;

/// 内容条目类型
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Blob,
    Lfs,
    Tree,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Blob => write!(f, "blob"),
            ContentType::Lfs => write!(f, "lfs"),
            ContentType::Tree => write!(f, "tree"),
        }
    }
}

/// 文件内容响应
#[derive(Debug, Deserialize)]
pub struct Content {
    /// 路径
    pub path: String,
    /// 类型
    #[serde(rename = "type")]
    pub content_type: ContentType,
    /// 文件内容（base64 编码，仅 blob 类型）
    #[serde(default)]
    pub content: String,
    /// LFS 下载 URL（仅 lfs 类型）
    #[serde(default)]
    pub lfs_download_url: String,
    /// 文件大小
    #[serde(default)]
    pub size: i64,
    /// 子条目（仅 tree 类型）
    #[serde(default)]
    pub entries: Vec<ContentEntry>,
}

/// 目录条目
#[derive(Debug, Deserialize)]
pub struct ContentEntry {
    /// 路径
    pub path: String,
    /// 类型
    #[serde(rename = "type")]
    pub entry_type: ContentType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_type_display() {
        assert_eq!(ContentType::Blob.to_string(), "blob");
        assert_eq!(ContentType::Lfs.to_string(), "lfs");
        assert_eq!(ContentType::Tree.to_string(), "tree");
    }

    #[test]
    fn content_type_deserialize() {
        let blob: ContentType =
            serde_json::from_str(r#""blob""#).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(blob, ContentType::Blob);

        let lfs: ContentType =
            serde_json::from_str(r#""lfs""#).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(lfs, ContentType::Lfs);

        let tree: ContentType =
            serde_json::from_str(r#""tree""#).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(tree, ContentType::Tree);
    }

    #[test]
    fn content_deserialize_blob() {
        let json = r#"{
            "path": "src/main.rs",
            "type": "blob",
            "content": "dGVzdA==",
            "size": 4
        }"#;
        let content: Content =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(content.path, "src/main.rs");
        assert_eq!(content.content_type, ContentType::Blob);
        assert_eq!(content.content, "dGVzdA==");
        assert_eq!(content.size, 4);
        assert!(content.entries.is_empty());
    }

    #[test]
    fn content_deserialize_tree_with_entries() {
        let json = r#"{
            "path": "src",
            "type": "tree",
            "entries": [
                {"path": "src/main.rs", "type": "blob"},
                {"path": "src/lib.rs", "type": "blob"}
            ]
        }"#;
        let content: Content =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(content.content_type, ContentType::Tree);
        assert_eq!(content.entries.len(), 2);
        assert_eq!(content.entries[0].path, "src/main.rs");
        assert_eq!(content.entries[0].entry_type, ContentType::Blob);
    }

    #[test]
    fn content_deserialize_defaults() {
        // 最小 JSON，验证 default 字段
        let json = r#"{"path": "test", "type": "blob"}"#;
        let content: Content =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("反序列化失败：{e}"));
        assert_eq!(content.content, "");
        assert_eq!(content.lfs_download_url, "");
        assert_eq!(content.size, 0);
        assert!(content.entries.is_empty());
    }
}
