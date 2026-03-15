# cnb-rs issue edit

```
cnb-rs issue edit <NUMBER> [flags]
```

编辑 Issue 信息。

支持修改标题、描述、优先级、开始日期和结束日期。至少需要指定一个修改项，
未指定的字段保持不变。

## 选项

- `<NUMBER>`: Issue 编号（必填）
- `-t, --title <TITLE>`: 修改标题
- `-b, --body <BODY>`: 修改描述
- `-p, --priority <PRIORITY>`: 修改优先级，可选值：`-2P`、`-1P`、`P0`、`P1`、`P2`、`P3`
- `--start-date <DATE>`: 修改开始日期，格式：`YYYY-MM-DD`
- `--end-date <DATE>`: 修改结束日期，格式：`YYYY-MM-DD`

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

::: warning
至少需要指定一个修改项，否则命令会报错退出。
:::

## 示例

```bash
# 修改标题
$ cnb-rs issue edit 42 -t "新标题"
✓ Issue #42 已更新

# 修改优先级
$ cnb-rs issue edit 42 -p P0
✓ Issue #42 已更新

# 同时修改多个字段
$ cnb-rs issue edit 42 -t "紧急修复" -p P0 -b "需要立即处理的问题"
✓ Issue #42 已更新

# 设置日期范围
$ cnb-rs issue edit 42 --start-date 2025-02-01 --end-date 2025-02-15
✓ Issue #42 已更新
```

## 错误处理

| 错误场景     | 错误信息                                                                        | 退出码 |
| ------------ | ------------------------------------------------------------------------------- | ------ |
| 未指定修改项 | `请至少指定一个修改项（--title、--body、--priority、--start-date、--end-date）` | 1      |
| Issue 不存在 | API 404 错误信息                                                                | 1      |
| 无权限       | API 403 错误信息                                                                | 1      |

## API

| 步骤       | API                                     | 方法  | 说明            |
| ---------- | --------------------------------------- | ----- | --------------- |
| 更新 Issue | `${API}/repos/{repo}/-/issues/{number}` | PATCH | 更新 Issue 信息 |

**请求体（仅包含指定的字段）：**

```json
{
  "title": "新标题",
  "body": "新描述",
  "priority": "P0",
  "start_date": "2025-02-01",
  "end_date": "2025-02-15"
}
```

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue view](/issue/view)
- [cnb-rs issue create](/issue/create)
