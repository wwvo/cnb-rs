//! AI 响应动作解析
//!
//! 从 AI 响应内容中解析 `get_api_doc` 或 `curl` 动作指令。

use regex_lite::Regex;
use std::sync::LazyLock;

/// AI 响应中的动作指令
#[derive(Debug)]
pub enum Action {
    /// 查询 API 文档，值为 "service/apiname"
    GetApiDoc(String),
    /// 执行 curl 命令（已合并续行符）
    Curl(String),
}

static DOC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)```get_api_doc\s*(.*?)```").unwrap_or_else(|_| unreachable!())
});
static CURL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)```bash\s*(curl.*?)```").unwrap_or_else(|_| unreachable!()));
static LINE_CONT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\\n\s*").unwrap_or_else(|_| unreachable!()));

/// 解析 AI 响应中的动作指令
///
/// 返回 `None` 表示 AI 已给出最终回答（无需继续循环）
pub fn parse_action(content: &str) -> Option<Action> {
    // 匹配 ```get_api_doc\n...\n```
    if let Some(caps) = DOC_RE.captures(content) {
        let value = caps[1].trim().to_string();
        return Some(Action::GetApiDoc(value));
    }

    // 匹配 ```bash\ncurl ...\n```
    if let Some(caps) = CURL_RE.captures(content) {
        let mut value = caps[1].trim().to_string();
        // 合并续行符（\ + 换行 + 可选空白）
        value = LINE_CONT_RE.replace_all(&value, " ").to_string();
        return Some(Action::Curl(value));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get_api_doc() {
        let content = "让我先查看文档：\n\n```get_api_doc\nissues/listissues\n```";
        match parse_action(content) {
            Some(Action::GetApiDoc(ref_str)) => assert_eq!(ref_str, "issues/listissues"),
            other => panic!("期望 GetApiDoc，得到 {other:?}"),
        }
    }

    #[test]
    fn test_parse_curl() {
        let content = r#"执行以下命令：

```bash
curl -X GET \
  "https://api.cnb.cool/repo/-/issues" \
  -H "Accept: application/vnd.cnb.api+json" \
  -H "Authorization: Bearer <CNB_TOKEN>"
```"#;
        match parse_action(content) {
            Some(Action::Curl(cmd)) => {
                assert!(cmd.contains("curl -X GET"));
                assert!(!cmd.contains('\n'));
            }
            other => panic!("期望 Curl，得到 {other:?}"),
        }
    }

    #[test]
    fn test_parse_final_answer() {
        let content = "根据查询结果，该仓库共有 5 个 Issue。";
        assert!(parse_action(content).is_none());
    }
}
