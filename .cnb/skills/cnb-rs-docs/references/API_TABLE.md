# API 表格生成规则

本文档定义从 Rust API 客户端代码提取 API 信息并生成文档表格的规则。

## 源码位置

- **客户端代码**：`crates/cnb-api/src/client/<module>.rs`
- **类型定义**：`crates/cnb-api/src/types/<module>.rs`

## 提取规则

### 1. 提取 API 路径

从客户端方法中提取 URL 格式：

```rust
pub async fn create_issue(&self, req: &CreateIssueRequest) -> Result<IssueDetail, ApiError> {
    let url = format!("{}{}/-/issues", self.base_url, self.repo);
    // ...
}
```

**提取结果：**
- 路径：`${API}/repos/{repo}/-/issues`
- 方法：POST（通过 `.post(&url)` 判断）

### 2. 确定 HTTP 方法

| 客户端调用 | HTTP 方法 |
|-----------|-----------|
| `.get(&url)` | GET |
| `.post(&url)` | POST |
| `.put(&url)` | PUT |
| `.patch(&url)` | PATCH |
| `.delete(&url)` | DELETE |

### 3. 提取请求体字段

从请求类型结构体提取：

```rust
pub struct CreateIssueRequest {
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}
```

**提取规则：**
- 非 `Option` 且无 `skip_serializing_if` → 必填
- `Option` 或有 `skip_serializing_if` → 可选

---

## 表格格式

### 标准格式

```markdown
## API

| 步骤 | API | 方法 | 说明 |
| ---- | --- | ---- | ---- |
| 创建 Issue | `${API}/repos/{repo}/-/issues` | POST | 创建新 Issue |
```

**字段说明：**
- **步骤**：动作描述（动词开头）
- **API**：URL 模板，用 `${API}` 前缀，`{repo}` 表示变量
- **方法**：GET/POST/PATCH/PUT/DELETE
- **说明**：简短描述

### 多步骤命令

某些命令涉及多个 API 调用：

```markdown
## API

| 步骤 | API | 方法 | 说明 |
| ---- | --- | ---- | ---- |
| 获取仓库信息 | `${API}/repos/{repo}` | GET | 获取 HEAD 分支 |
| 创建 PR | `${API}/repos/{repo}/-/pulls` | POST | 创建 Pull Request |
```

### 请求体字段

在表格后列出请求体字段：

```markdown
**请求体字段：** `title`（必填）、`body`、`priority`、`labels`、`assignees`、`start_date`、`end_date`
```

---

## 完整示例

### 源码

```rust
// crates/cnb-api/src/client/issue.rs
pub async fn create_issue(&self, req: &CreateIssueRequest) -> Result<IssueDetail, ApiError> {
    let url = format!("{}{}/-/issues", self.base_url, self.repo);
    let resp = self.http.post(&url).json(req).send().await?;
    Self::handle_response(resp).await
}

// crates/cnb-api/src/types/issue.rs
pub struct CreateIssueRequest {
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub priority: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
```

### 生成文档

```markdown
## API

| 步骤 | API | 方法 | 说明 |
| ---- | --- | ---- | ---- |
| 创建 Issue | `${API}/repos/{repo}/-/issues` | POST | 创建新 Issue |

**请求体字段：** `title`（必填）、`body`、`priority`、`labels`、`assignees`、`start_date`、`end_date`
```

---

## 路径变量替换

| URL 变量 | 含义 |
|----------|------|
| `{repo}` | 仓库路径（`group/repo`） |
| `{number}` | Issue/PR 编号 |
| `{branch}` | 分支名称 |
| `{tag}` | 标签名称 |
| `{name}` | 资源名称 |

### 示例

```rust
let url = format!("{}{}/-/issues/{number}", self.base_url, self.repo);
```

生成：`${API}/repos/{repo}/-/issues/{number}`

---

## 特殊情况

### 无 API 调用的命令

如 `cnb browse`、`cnb completion`，不生成 API 部分。

### 分页查询

对于列表类 API，在说明中注明分页：

```markdown
| 步骤 | API | 方法 | 说明 |
| ---- | --- | ---- | ---- |
| 列出 Issue | `${API}/repos/{repo}/-/issues` | GET | 支持分页和过滤 |
```

### 认证相关

登录命令不暴露 API 详情：

```markdown
## API

认证相关 API，详见 [认证文档](/auth/)。
```

---

## base_url 变体

项目中使用不同的 base_url：

| 变量 | 用途 |
|------|------|
| `self.base_url` | API 基础 URL（`${API}`） |
| `self.base_web_url` | Web 页面 URL（用于输出链接） |

生成的 API 表格统一使用 `${API}` 前缀。

---

## 从 client 代码推断操作描述

| 方法名模式 | 步骤描述 |
|-----------|----------|
| `list_*` | 列出/获取列表 |
| `get_*` | 获取 |
| `create_*` | 创建 |
| `update_*` | 更新 |
| `delete_*` | 删除 |
| `add_*` | 添加 |
| `remove_*` | 移除 |
| `merge_*` | 合并 |

---

## 质量检查

生成 API 表格后，检查：

1. ✅ 路径格式正确（`${API}/...`）
2. ✅ 方法大写
3. ✅ 步骤动词准确
4. ✅ 必填字段标注正确
5. ✅ 字段名使用 snake_case（与 API 一致）
