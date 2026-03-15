# 命令文档模板

本文档定义 cnb-rs CLI 命令文档的标准结构和格式。

## 文件命名与位置

- **文件路径**：`docs/commands/<command>/<subcommand>.md`
- **索引文件**：`docs/commands/<command>/index.md`

示例：
- `docs/commands/issue/create.md` - 创建 Issue 的文档
- `docs/commands/issue/index.md` - issue 命令组索引

---

## 子命令文档模板

```markdown
# cnb-rs <command> <subcommand>

`cnb-rs <command> <subcommand> [flags]`

<一句话描述命令功能>

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
$ cnb-rs <command> <subcommand> <required-args>
<expected-output>

# 带选项用法
$ cnb-rs <command> <subcommand> --option value

# 完整示例
$ cnb-rs <command> <subcommand> -s value --long value
```

## API

| 步骤 | API | 方法 | 说明 |
| ---- | --- | ---- | ---- |
| 动作 | `${API}/path` | GET | 描述 |

**请求体字段：** `field1`（必填）、`field2`

## 另请参阅

- [cnb-rs <command>](/commands/<command>/)
- [cnb-rs <command> <related>](/commands/<command>/<related>/)
```

---

## 各部分详解

### 标题部分

```markdown
# cnb-rs <command> <subcommand>

`cnb-rs <command> <subcommand> [flags]`

<一句话描述命令功能>
```

**规则：**

1. 标题格式：`cnb-rs <command> <subcommand>`
2. 命令格式：使用反引号，包含 `[flags]` 表示可选参数
3. 描述：简洁明了，一句话说明功能

### 选项部分

```markdown
## 选项

- `-t, --title <TITLE>`: Issue 标题（必填）
- `-b, --body <BODY>`: Issue 描述（默认：空）
- `-p, --priority <PRIORITY>`: 优先级，可选值：`P0`、`P1`、`P2`、`P3`
- `-l, --labels <LABELS>`: 标签名称，多个用逗号分隔
- `--start-date <DATE>`: 开始日期，格式：`YYYY-MM-DD`
```

**格式规则：**

1. 短参数+长参数：`-s, --long <VALUE>`
2. 仅长参数：`--long <VALUE>`
3. 布尔标志：`--flag`（无 `<VALUE>`）
4. 默认值：`（默认：value）`
5. 必填标记：`（必填）`
6. 可选值：`可选值：\`a\`、\`b\`、\`c\``
7. 格式说明：用反引号包裹格式字符串

### 示例部分

```markdown
## 示例

```bash
# 创建简单 Issue
$ cnb-rs issue create -t "修复登录问题"
https://cnb.cool/org/repo/-/issues/42

# 创建带描述的 Issue
$ cnb-rs issue create -t "新功能需求" -b "详细描述..."

# 创建带完整信息的 Issue
$ cnb-rs issue create -t "紧急 Bug" -b "页面崩溃" -p P0 -l "bug,urgent" -a "zhangsan,lisi"
```
```

**规则：**

1. 每个示例以 `#` 注释开头说明目的
2. 命令用 `$ cnb-rs` 前缀
3. 包含典型输出（如有）
4. 从简单到复杂排列

### API 部分

```markdown
## API

| 步骤       | API                            | 方法 | 说明         |
| ---------- | ------------------------------ | ---- | ------------ |
| 创建 Issue | `${API}/repos/{repo}/-/issues` | POST | 创建新 Issue |

**请求体字段：** `title`（必填）、`body`、`priority`、`labels`、`assignees`、`start_date`、`end_date`
```

**规则：**

1. API 路径用 `` `${API}/path` `` 格式
2. `{repo}` 表示路径参数
3. 方法用大写：GET、POST、PATCH、PUT、DELETE
4. 请求体字段列出所有字段，必填字段标注 `（必填）`

### 另请参阅部分

```markdown
## 另请参阅

- [cnb-rs issue](/commands/issue/)
- [cnb-rs issue view](/commands/issue/view/)
- [cnb-rs issue edit](/commands/issue/edit/)
```

**规则：**

1. 链接到父命令索引页
2. 链接到相关命令
3. VitePress 内部链接格式：`[text](/path/)`

---

## 特殊情况

### 无子命令的命令

如 `cnb-rs browse`、`cnb-rs info`：

```markdown
# cnb-rs browse

`cnb-rs browse [flags]`

在浏览器中打开仓库页面。
```

### 索引页面（index.md）

命令组的索引页列出所有子命令：

```markdown
# cnb-rs issue

Issue 管理。

## 子命令

| 命令 | 说明 |
| ---- | ---- |
| [list](list) | 列出仓库的 Issue |
| [create](create) | 创建 Issue |
| [view](view) | 查看 Issue 详情 |
| [edit](edit) | 编辑 Issue |
| [close](close) | 关闭 Issue |
| [reopen](reopen) | 重新打开 Issue |
```
