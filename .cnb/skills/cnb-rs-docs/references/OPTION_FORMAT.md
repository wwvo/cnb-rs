# 选项格式规范

本文档定义从 Rust clap 参数到文档选项的映射规则。

## 参数类型映射

| Rust 类型 | 文档格式 | 示例 |
|-----------|----------|------|
| `String` | `<VALUE>`（必填） | `--title <TITLE>` |
| `Option<String>` | `<VALUE>`（可选） | `--start-date <DATE>` |
| `Vec<String>` | `<VALUES>`（多值） | `--labels <LABELS>` |
| `bool` | 无值（标志） | `--private` |
| `u32`/`i32` | `<NUMBER>` | `--page <PAGE>` |

## clap 属性映射

### 短参数（short）

```rust
#[arg(short = 't')]
```
生成：`-t`

**特殊：** `#[arg(short)]` 自动使用字段名首字母

### 长参数（long）

```rust
#[arg(long = "title")]
```
生成：`--title`

**特殊：** `#[arg(long)]` 自动使用字段名（下划线转连字符）

### 默认值（default_value）

```rust
#[arg(default_value = "")]
```
生成：`（默认：空）`

```rust
#[arg(default_value = "open")]
```
生成：`（默认：open）`

### 值分隔符（value_delimiter）

```rust
#[arg(value_delimiter = ',')]
```
生成：说明中添加 `多个用逗号分隔`

---

## 完整示例解析

### 示例 1：必填参数

```rust
/// Issue 标题
#[arg(short = 't', long = "title")]
pub title: String,
```

**生成文档：**
```
- `-t, --title <TITLE>`: Issue 标题（必填）
```

### 示例 2：带默认值

```rust
/// Issue 描述
#[arg(short = 'b', long = "body", default_value = "")]
pub body: String,
```

**生成文档：**
```
- `-b, --body <BODY>`: Issue 描述（默认：空）
```

### 示例 3：可选参数

```rust
/// 开始日期（格式：YYYY-MM-DD）
#[arg(long = "start-date")]
pub start_date: Option<String>,
```

**生成文档：**
```
- `--start-date <DATE>`: 开始日期，格式：`YYYY-MM-DD`
```

### 示例 4：多值参数

```rust
/// 标签（逗号分隔）
#[arg(short = 'l', long = "labels", value_delimiter = ',')]
pub labels: Vec<String>,
```

**生成文档：**
```
- `-l, --labels <LABELS>`: 标签名称，多个用逗号分隔
```

### 示例 5：布尔标志

```rust
/// 创建私有仓库
#[arg(long)]
pub private: bool,
```

**生成文档：**
```
- `--private`: 创建私有仓库
```

### 示例 6：位置参数

```rust
/// 仓库名称
pub name: String,
```

**生成文档：**
```
- `<name>`: 仓库名称（必填）
```

### 示例 7：短参数自动推断

```rust
/// 所属组织
#[arg(short, long)]
pub group: Option<String>,
```

**生成文档：**
```
- `-g, --group <GROUP>`: 所属组织
```

### 示例 8：枚举可选值

若代码中有枚举或文档注释说明可选值：

```rust
/// 优先级
#[arg(short = 'p', long = "priority", default_value = "")]
pub priority: String,
```

结合业务逻辑或 API 文档，生成：
```
- `-p, --priority <PRIORITY>`: 优先级，可选值：`-2P`、`-1P`、`P0`、`P1`、`P2`、`P3`
```

---

## VALUE 命名约定

参数占位符使用大写，遵循以下约定：

| 类型 | 占位符 |
|------|--------|
| 标题 | `<TITLE>` |
| 名称 | `<NAME>` |
| 描述 | `<TEXT>` / `<BODY>` |
| 分支 | `<BRANCH>` |
| 仓库 | `<REPO>` |
| 日期 | `<DATE>` |
| 数量 | `<COUNT>` / `<NUMBER>` |
| 页码 | `<PAGE>` |
| 标签列表 | `<LABELS>` |
| 用户列表 | `<USERS>` / `<ASSIGNEES>` |
| 文件路径 | `<FILE>` / `<PATH>` |
| URL | `<URL>` |
| ID | `<ID>` / `<NUMBER>` |

---

## 注释提取规则

### 单行注释

```rust
/// Issue 标题
#[arg(short = 't', long = "title")]
pub title: String,
```

提取：`Issue 标题`

### 多行注释

```rust
/// 优先级
///
/// 可选值：-2P、-1P、P0、P1、P2、P3
#[arg(short = 'p', long = "priority")]
pub priority: String,
```

提取：`优先级。可选值：-2P、-1P、P0、P1、P2、P3`

### 带格式说明

```rust
/// 开始日期（格式：YYYY-MM-DD）
#[arg(long = "start-date")]
pub start_date: Option<String>,
```

提取：`开始日期，格式：\`YYYY-MM-DD\``

---

## 选项排序

1. **位置参数**：按代码定义顺序
2. **选项参数**：按短参数字母顺序（无短参数的排在后面）
3. **全局选项**：固定顺序，放在最后

---

## 特殊处理

### 隐藏参数

```rust
#[arg(hide = true)]
```

**不生成文档**

### 冲突参数

```rust
#[arg(conflicts_with = "private")]
```

在说明中注明冲突关系（如有必要）

### 互斥组

```rust
#[arg(group = "visibility")]
pub private: bool,

#[arg(group = "visibility")]
pub secret: bool,
```

在说明中注明：`--private` 与 `--secret` 互斥

---

## 常见错误

❌ **错误**：`-t --title <TITLE>`（缺少逗号）

✅ **正确**：`-t, --title <TITLE>`

❌ **错误**：`--title <title>`（占位符应大写）

✅ **正确**：`--title <TITLE>`

❌ **错误**：`--private <BOOL>`（布尔标志不应有值）

✅ **正确**：`--private`

❌ **错误**：默认值用引号：`（默认："open"）`

✅ **正确**：`（默认：open）` 或 `（默认：\`open\`）`
