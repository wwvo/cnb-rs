# cnb-rs build list

```
cnb-rs build list [options]
```

列出仓库构建记录，支持多种过滤条件。

## 选项

- `-s, --status <STATUS>`: 按状态过滤：`pending`、`success`、`error`、`cancel`
- `-e, --event <EVENT>`: 按事件过滤
- `-b, --branch <BRANCH>`: 按源分支过滤
- `-u, --user <USERNAME>`: 按用户过滤
- `--since <DATE>`: 开始日期（`YYYY-MM-DD`）
- `--until <DATE>`: 结束日期（`YYYY-MM-DD`）
- `-n, --limit <N>`: 每页数量（默认：`30`）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出最近构建
$ cnb-rs build list

# 过滤失败的构建
$ cnb-rs build list --status error

# 按分支和日期过滤
$ cnb-rs build list --branch main --since 2025-01-01

# JSON 格式输出
$ cnb-rs build list --json
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build status](/build/status)
- [cnb-rs build start](/build/start)
