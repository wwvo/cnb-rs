//! `OpenAPI` 嵌入式文档检索
//!
//! 通过 build.rs 编译期嵌入 references/ 目录下的 API 文档，
//! 运行时按 `service/apiname` 格式检索。

// 引入 build.rs 生成的嵌入式文档代码
include!(concat!(env!("OUT_DIR"), "/embedded_refs.rs"));

/// 获取指定 API 的详细文档
///
/// 支持 `service/apiname` 格式，如 `issues/listissues`。
///
/// # Errors
///
/// Returns an error string if `doc_ref` is not a valid `service/apiname`
/// reference or the referenced document does not exist.
pub fn get_api_doc(doc_ref: &str) -> Result<String, String> {
    let parts: Vec<&str> = doc_ref.splitn(2, '/').collect();
    if parts.len() != 2 {
        return Err(format!(
            "无效的文档引用 \"{doc_ref}\"，格式应为 \"service/apiname\"，如 \"issues/listissues\""
        ));
    }

    if let Some(content) = get_embedded_doc(doc_ref) {
        Ok(content.to_string())
    } else {
        let (service, api_name) = (parts[0], parts[1]);
        let available = list_apis(service);
        if available.is_empty() {
            let services = list_services();
            Err(format!(
                "服务 \"{}\" 不存在。可用服务：{}",
                service,
                services.join(", ")
            ))
        } else {
            Err(format!(
                "API \"{}\" 在服务 \"{}\" 中不存在。可用：{}",
                api_name,
                service,
                available.join(", ")
            ))
        }
    }
}

/// 列出所有 API 服务分类
pub fn list_services() -> Vec<String> {
    list_embedded_services()
        .iter()
        .copied()
        .map(String::from)
        .collect()
}

/// 列出指定服务下的所有 API 名称
pub fn list_apis(service: &str) -> Vec<String> {
    let prefix = format!("{service}/");
    list_embedded_keys()
        .iter()
        .filter(|key| key.starts_with(&prefix))
        .filter_map(|key| key.strip_prefix(&prefix))
        .map(String::from)
        .collect()
}
