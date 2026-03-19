//! curl 命令解析与 reqwest 直接执行
//!
//! 将 AI 生成的 curl 命令解析为 HTTP 请求参数，用 reqwest 直接发请求，
//! 无需依赖外部 curl/shell。

use regex_lite::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::sync::LazyLock;

/// curl 执行结果
#[derive(Debug, Serialize)]
pub struct CurlResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 解析后的 curl 命令参数
struct CurlCommand {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

/// 解析 curl 命令字符串为结构化参数
fn parse_curl(cmd: &str) -> Result<CurlCommand, String> {
    let mut method = String::from("GET");
    let mut url = String::new();
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut body: Option<String> = None;

    // 将命令按空格拆分，但保留引号内的内容
    let tokens = tokenize(cmd);

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token.as_str() {
            "curl" | "-s" | "--silent" | "-S" | "--show-error" => {
                // 跳过 curl 自身和静默标志
            }
            "-X" | "--request" => {
                i += 1;
                if i < tokens.len() {
                    method = tokens[i].to_uppercase();
                }
            }
            "-H" | "--header" => {
                i += 1;
                if i < tokens.len() {
                    let header = unquote(&tokens[i]);
                    if let Some((key, value)) = header.split_once(':') {
                        headers.push((key.trim().to_string(), value.trim().to_string()));
                    }
                }
            }
            "-d" | "--data" | "--data-raw" => {
                i += 1;
                if i < tokens.len() {
                    body = Some(unquote(&tokens[i]));
                }
            }
            _ => {
                // 没有 flag 前缀的 token 视为 URL
                if !token.starts_with('-') && url.is_empty() {
                    url = unquote(token);
                }
            }
        }
        i += 1;
    }

    // 如果有 body 但没有显式指定 method，默认为 POST
    if body.is_some() && method == "GET" {
        method = "POST".to_string();
    }

    if url.is_empty() {
        return Err("curl 命令中未找到 URL".to_string());
    }

    Ok(CurlCommand {
        method,
        url,
        headers,
        body,
    })
}

/// 执行 curl 命令（通过 reqwest 直接发 HTTP 请求）
///
/// `vars` 为占位符替换映射，如 `{"<CNB_TOKEN>": "xxx"}`
pub async fn exec_curl<S>(
    http: &reqwest::Client,
    curl_cmd: &str,
    vars: &HashMap<String, String, S>,
) -> CurlResult
where
    S: BuildHasher,
{
    // 替换占位符
    let mut cmd = curl_cmd.to_string();
    for (placeholder, value) in vars {
        cmd = cmd.replace(placeholder, value);
    }

    // 解析 curl 命令
    let parsed = match parse_curl(&cmd) {
        Ok(p) => p,
        Err(e) => {
            return CurlResult {
                success: false,
                data: None,
                error: Some(e),
            };
        }
    };

    let method = match parsed.method.as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "PATCH" => reqwest::Method::PATCH,
        "DELETE" => reqwest::Method::DELETE,
        other => {
            return CurlResult {
                success: false,
                data: None,
                error: Some(format!("不支持的 HTTP 方法：{other}")),
            };
        }
    };

    let mut req = http.request(method, &parsed.url);

    for (key, value) in &parsed.headers {
        req = req.header(key.as_str(), value.as_str());
    }

    if let Some(ref body) = parsed.body {
        req = req.body(body.clone());
    }

    // 发送请求
    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();

            // 尝试解析为 JSON
            let data: serde_json::Value =
                serde_json::from_str(&text).unwrap_or(serde_json::Value::String(text));

            if (200..300).contains(&status) {
                CurlResult {
                    success: true,
                    data: Some(data),
                    error: None,
                }
            } else {
                CurlResult {
                    success: false,
                    data: Some(data),
                    error: Some(format!("HTTP {status}")),
                }
            }
        }
        Err(e) => CurlResult {
            success: false,
            data: None,
            error: Some(format!("请求失败：{e}")),
        },
    }
}

/// 将命令行字符串拆分为 token 列表，保留引号内内容为完整 token
fn tokenize(cmd: &str) -> Vec<String> {
    static TOKENIZE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"'[^']*'|"[^"]*"|\S+"#).unwrap_or_else(|_| unreachable!()));

    TOKENIZE_RE
        .find_iter(cmd)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// 去除字符串两端的引号（单引号或双引号）
fn unquote(s: &str) -> String {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_get() {
        let cmd = r#"curl -X GET "https://api.cnb.cool/repo/-/issues" -H "Accept: application/vnd.cnb.api+json" -H "Authorization: Bearer token123""#;
        let parsed = parse_curl(cmd).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(parsed.method, "GET");
        assert_eq!(parsed.url, "https://api.cnb.cool/repo/-/issues");
        assert_eq!(parsed.headers.len(), 2);
        assert!(parsed.body.is_none());
    }

    #[test]
    fn test_parse_post_with_body() {
        let cmd = r#"curl -X POST "https://api.cnb.cool/repo/-/issues" -H "Content-Type: application/json" -d '{"title":"test"}'"#;
        let parsed = parse_curl(cmd).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.body.as_deref(), Some(r#"{"title":"test"}"#));
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize(r#"curl -X GET "https://example.com" -H "Accept: text/html""#);
        assert_eq!(
            tokens,
            vec![
                "curl",
                "-X",
                "GET",
                "\"https://example.com\"",
                "-H",
                "\"Accept: text/html\""
            ]
        );
    }

    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"hello\""), "hello");
        assert_eq!(unquote("'world'"), "world");
        assert_eq!(unquote("noquotes"), "noquotes");
    }
}
