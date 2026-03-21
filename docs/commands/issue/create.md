---
title: cnb-rs issue create
---

# cnb-rs issue create

```
cnb-rs issue create [flags]
```

创建 Issue。

创建成功后输出新建 Issue 的 Web 链接。支持设置标题、描述、优先级、标签、
处理人以及开始/结束日期。

## 选项

- `-t, --title <TITLE>`: Issue 标题（必填）
- `-b, --body <BODY>`: Issue 描述（默认：空）
- `-p, --priority <PRIORITY>`: 优先级，可选值：`-2P`、`-1P`、`P0`、`P1`、`P2`、`P3`
- `-l, --labels <LABELS>`: 标签名称，多个用逗号分隔
- `-a, --assignees <ASSIGNEES>`: 处理人用户名，多个用逗号分隔
- `--start-date <DATE>`: 开始日期，格式：`YYYY-MM-DD`
- `--end-date <DATE>`: 结束日期，格式：`YYYY-MM-DD`

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 创建简单 Issue
$ cnb-rs issue create -t "修复登录问题"
https://cnb.cool/org/repo/-/issues/42

# 创建带描述的 Issue
$ cnb-rs issue create -t "新功能需求" -b "详细描述..."

# 创建带完整信息的 Issue
$ cnb-rs issue create -t "紧急 Bug" -b "页面崩溃" -p P0 -l "bug,urgent" -a "zhangsan,lisi"

# 指定日期范围
$ cnb-rs issue create -t "Sprint 任务" --start-date 2025-01-01 --end-date 2025-01-15
```

## API

| 步骤       | API                            | 方法 | 说明         |
| ---------- | ------------------------------ | ---- | ------------ |
| 创建 Issue | `${API}/repos/{repo}/-/issues` | POST | 创建新 Issue |

**请求体字段：** `title`（必填）、`body`、`priority`、`labels`、`assignees`、`start_date`、`end_date`

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue view](/issue/view)
- [cnb-rs issue edit](/issue/edit)
