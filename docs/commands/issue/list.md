# cnb-rs issue list

```
cnb-rs issue list [flags]
```

列出仓库的 Issue。

默认列出状态为 `open` 的 Issue，支持通过多种条件过滤和排序。
输出为表格格式，包含 Issue 编号、标题、优先级、作者、最后活动时间和不活跃天数。

## 选项

- `-s, --state <STATE>`: 状态过滤，可选值：`open`、`closed`（默认：`open`）
- `-k, --keyword <KEYWORD>`: 关键词搜索，匹配 Issue 标题和描述
- `-p, --priority <PRIORITY>`: 优先级过滤，如 `P0`、`P1`（多个用逗号分隔）
- `-l, --labels <LABELS>`: 标签过滤（多个用逗号分隔）
- `--author <AUTHOR>`: 创建者用户名过滤
- `-a, --assignee <ASSIGNEE>`: 处理人过滤（`-` 表示未分配）
- `--sort <SORT>`: 排序字段，支持 `created_at`、`-created_at`、`-updated_at`、`-last_acted_at`（前缀 `-` 表示倒序）
- `-L, --limit <N>`: 最大返回数量（默认：`30`，最大 `100`）
- `-d, --stale-days <N>`: 仅显示超过 N 天没有活动的 Issue（默认：`0`，即不过滤）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出所有 open 状态的 Issue
$ cnb-rs issue list

# 列出已关闭的 Issue
$ cnb-rs issue list -s closed

# 搜索包含关键词的 Issue
$ cnb-rs issue list -k "登录"

# 过滤 P0 优先级
$ cnb-rs issue list -p P0

# 过滤标签
$ cnb-rs issue list -l "bug,urgent"

# 过滤未分配的 Issue
$ cnb-rs issue list -a -

# 按创建时间倒序，取前 10 条
$ cnb-rs issue list --sort -created_at -L 10

# 显示超过 30 天没有活动的 Issue
$ cnb-rs issue list -d 30

# JSON 格式输出
$ cnb-rs --json issue list
```

## API

| 步骤       | API                            | 方法 | 说明            |
| ---------- | ------------------------------ | ---- | --------------- |
| 列出 Issue | `${API}/repos/{repo}/-/issues` | GET  | 获取 Issue 列表 |

**查询参数：** `state`、`page`、`page_size`、`keyword`、`priority`、`labels`、`authors`、`assignees`、`order_by`

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue mine](/issue/mine)
- [cnb-rs issue view](/issue/view)
