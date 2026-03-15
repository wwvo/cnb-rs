---
name: cnb-rs-docs
description: Generate VitePress docs for cnb-rs CLI commands. Analyzes Rust source (clap args) to extract options, examples, and API info. Use when generating or updating command documentation for cnb-rs project.
license: Apache-2.0
metadata:
  author: cnb-cli
  version: "1.0.0"
compatibility: 适用于 cnb-rs 项目（Rust + clap），生成 VitePress 格式文档
allowed-tools: Read Glob Grep
---

# cnb-rs-docs Skill

为 cnb-rs CLI 项目生成 VitePress 格式的命令文档。

## 工作流程

### 1. 分析命令结构

从 `src/commands/<command>/` 读取 mod.rs 和 *.rs 文件，识别：

- **子命令枚举**：从 `mod.rs` 的 `#[derive(Subcommand)]` 提取子命令列表
- **参数定义**：从 `#[derive(Args)]` 结构体提取参数属性
- **命令路由**：从 `match subcommand` 理解命令分发逻辑

### 2. 提取参数信息

解析 clap 属性：

| Rust 属性 | 文档格式 |
|-----------|----------|
| `#[arg(short = 'x')]` | `-x` |
| `#[arg(long = "xxx")]` | `--xxx` |
| `#[arg(default_value = "v")]` | `（默认：v）` |
| `/// 文档注释` | 参数说明 |

详见 [OPTION_FORMAT.md](references/OPTION_FORMAT.md)

### 3. 生成文档

按模板生成 VitePress 格式 Markdown：

```markdown
# cnb-rs <command> [subcommand]

`cnb-rs <command> [subcommand] [flags]`

<一句话描述>

## 选项

- `-s, --short <VALUE>`: 参数说明（默认：`default`）
- `--long <VALUE>`: 参数说明

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 基本用法
$ cnb-rs <command> <args>

# 带选项用法
$ cnb-rs <command> --option value
```

## API

| 步骤 | API | 方法 | 说明 |
|------|-----|------|------|
| 动作 | `${API}/path` | GET | 描述 |

**请求体字段：** `field1`（必填）、`field2`

## 另请参阅

- [cnb-rs <parent>](/parent/)
```

详见 [COMMAND_TEMPLATE.md](references/COMMAND_TEMPLATE.md) 和 [API_TABLE.md](references/API_TABLE.md)

## 触发示例

以下指令可以触发此 skill：

- "为 cnb-rs issue create 生成文档"
- "更新 cnb-rs pr list 的文档"
- "检查 docs/commands/issue/create.md 是否需要更新"
- "为新增的 cnb-rs repo fork 命令生成文档"
- "生成 cnb-rs 所有命令的文档"

## 使用示例

当用户请求 **"为 cnb-rs issue create 生成文档"** 时，执行流程：

**1. 读取源码：**
- `src/commands/issue/mod.rs` - 获取子命令列表
- `src/commands/issue/create.rs` - 获取参数定义
- `crates/cnb-api/src/types/issue.rs` - 获取 API 请求结构

**2. 提取信息：**
```rust
/// Issue 标题
#[arg(short = 't', long = "title")]
pub title: String,
```

提取为：`-t, --title <TITLE>`: Issue 标题（必填）

**3. 生成文档：**
在 `docs/commands/issue/create.md` 创建文档文件。

## 关键规则

### 选项格式
- 短参数+长参数：`-s, --long <VALUE>`
- 仅长参数：`--long <VALUE>`
- 布尔标志：`--flag`（无 `<VALUE>`）
- 默认值：`（默认：value）`
- 必填标记：`（必填）`

### 示例格式
- 使用 `$` 前缀
- 注释说明目的
- 从简单到复杂排列

### API 表格
- 路径格式：`${API}/repos/{repo}/path`
- 变量表示：`{repo}`、`{number}`
- 方法大写：GET、POST、PATCH、PUT、DELETE

### 风格一致性
与 `docs/commands/` 现有文档保持一致。

## 参考资料

- [命令文档模板](references/COMMAND_TEMPLATE.md)
- [选项格式规范](references/OPTION_FORMAT.md)
- [API 表格规则](references/API_TABLE.md)
- [示例片段库](assets/examples/)

## 全局选项

以下选项在所有命令中通用（继承自 `GlobalArgs`）：

| 选项 | 类型 | 说明 |
|------|------|------|
| `--repo <REPO>` | String | 指定仓库路径（格式：`group/repo`） |
| `--json` | bool | 以 JSON 格式输出 |
| `--domain <DOMAIN>` | String | 指定目标域名（默认：`cnb.cool`） |

## 注意事项

1. **优先读取源码** - 不依赖外部文档，直接从 Rust 代码提取信息
2. **保持简洁** - 文档描述应简明扼要，避免冗余
3. **示例实用** - 示例应覆盖常见用例，从简单到复杂
4. **API 准确** - API 路径和方法必须与客户端代码一致
5. **交叉引用** - 在"另请参阅"中链接相关命令

## 错误处理

当遇到以下情况时：

- **源码文件不存在**：提示用户确认命令名称或路径是否正确
- **参数解析失败**：建议检查 clap 属性是否标准，检查 `#[derive(Args)]` 是否正确
- **API 信息缺失**：仅生成选项和示例部分，跳过 API 表格
- **类型定义复杂**：优先提取基本类型字段，复杂嵌套类型可简化描述

## 版本历史

### 1.0.0

- 初始版本
- 支持 issue、pr、repo 等命令的文档生成
- 提供完整的模板和规范文档

## 执行权限

此 Skill 仅使用只读工具（Read、Glob、Grep），不会修改任何文件。生成的文档内容由 AI 助手输出，由用户决定如何使用。
