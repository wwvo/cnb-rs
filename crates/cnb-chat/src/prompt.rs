//! System Prompt 构建
//!
//! 从 SKILL.md 提取精简 API 索引，构建两阶段检索的 System Prompt。

use regex_lite::Regex;
use std::sync::LazyLock;

/// 嵌入 SKILL.md 内容
const SKILL_CONTENT: &str = include_str!("../SKILL.md");

/// 默认 API 基础地址
const DEFAULT_API_ENDPOINT: &str = "https://api.cnb.cool";

/// 获取 API 基础地址（支持环境变量覆盖）
#[must_use]
pub fn get_api_endpoint() -> String {
    std::env::var("CNB_API_ENDPOINT")
        .unwrap_or_else(|_| DEFAULT_API_ENDPOINT.to_string())
        .trim_end_matches('/')
        .to_string()
}

/// 从 SKILL.md 生成精简索引（每个 API 一行）
///
/// 格式：`- APIName: 描述 [service/apiname]`
pub fn get_compact_index() -> String {
    static SERVICE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^### (.+?) 服务$").unwrap_or_else(|_| unreachable!()));
    static API_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^#### (.+)$").unwrap_or_else(|_| unreachable!()));
    static DESC_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\*\*描述：\*\* (.+)$").unwrap_or_else(|_| unreachable!()));
    static DOC_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\*\*详细文档：\*\*").unwrap_or_else(|_| unreachable!()));

    let mut current_service = String::new();
    let mut result: Vec<String> = Vec::new();
    let mut api_name = String::new();
    let mut description = String::new();

    for line in SKILL_CONTENT.lines() {
        if let Some(caps) = SERVICE_RE.captures(line) {
            if !current_service.is_empty() {
                result.push(String::new());
            }
            current_service = caps[1].to_string();
            result.push(format!("### {current_service}"));
            continue;
        }
        if let Some(caps) = API_RE.captures(line) {
            api_name = caps[1].to_string();
            continue;
        }
        if let Some(caps) = DESC_RE.captures(line) {
            // 只取第一句（句号前）
            description = caps[1].split('。').next().unwrap_or("").to_string();
            continue;
        }
        if DOC_RE.is_match(line) && !api_name.is_empty() {
            result.push(format!(
                "- {api_name}: {description} [{}/{}]",
                current_service,
                api_name.to_lowercase()
            ));
            api_name.clear();
            description.clear();
        }
    }

    result.join("\n")
}

/// 构建两阶段检索的 System Prompt
#[must_use]
pub fn build_system_prompt() -> String {
    let endpoint = get_api_endpoint();
    let compact_index = get_compact_index();

    format!(
        r#"你是一个 CNB 平台 Agent，能够通过调用 CNB OpenAPI 来完成用户的请求。

## API 基础配置

- API 基础地址：{endpoint}
- 认证方式：Bearer Token
- 请求头：Accept: application/vnd.cnb.api+json, Authorization: Bearer <CNB_TOKEN>

## API 接口索引

以下是所有可用的 API 列表。每行格式：`API 名称：描述 [文档引用]`
方括号中的 `[service/apiname]` 是文档引用标识，用于获取详细文档。

{compact_index}

## 工作流程

1. 用户提出需求后，分析需求并确定需要哪些 API。
2. **获取 API 详细文档**：如果你需要了解某个 API 的详细参数、请求体格式或响应结构，请输出文档查询指令：

```get_api_doc
service/apiname
```

例如，要查看 ListIssues 的详细文档：
```get_api_doc
issues/listissues
```

系统会返回该 API 的完整文档（包含参数、请求体、响应格式、cURL 示例等）。

3. **调用 API**：获取到详细文档后，输出可执行的 curl 命令，用 ```bash 代码块包裹：

```bash
curl -X GET \
  "{endpoint}/{{repo}}/-/issues" \
  -H "Accept: application/vnd.cnb.api+json" \
  -H "Authorization: Bearer <CNB_TOKEN>"
```

4. 系统会执行该 curl 命令并将结果返回给你。
5. 你根据返回结果继续分析，可以继续获取文档或调用 API，或者给出最终回答。
6. 当你要给出最终回答时，正常使用文字回复即可（不要输出代码块）。

## 重要规则

- 在调用不熟悉的 API 前，**先用 get_api_doc 获取详细文档**，确认参数格式
- 路径中的变量要替换为实际值，如 `{{repo}}` 替换为实际仓库路径
- 认证 token 统一使用占位符 `<CNB_TOKEN>`，系统会自动替换
- 每次只输出一个代码块（get_api_doc 或 bash）
- 如果不确定参数，先调用 GET 接口查询信息
"#
    )
}
