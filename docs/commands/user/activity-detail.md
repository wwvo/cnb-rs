---
title: cnb-rs user activity-detail
---

# cnb-rs user activity-detail

```
cnb-rs user activity-detail [<username>] --type <type> --repo <repo> --date <date>
```

查看用户在指定仓库中某类活动的详细记录。

根据活动类型返回不同的详细数据，例如 commit 类型会列出每次提交的 SHA 和消息，
PR 类型会列出每个 Pull Request 的编号和标题等。

## 选项

- `[<username>]`: 用户名（不指定则查看当前用户）
- `-t, --type <TYPE>`: 活动类型：`commit`、`pr`、`issue`、`review`（必填）
- `-r, --repo <REPO>`: 仓库路径（必填）
- `-d, --date <DATE>`: 查询日期，格式 `yyyyMM`（按月）或 `yyyyMMdd`（按日）（必填）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前用户在指定仓库的提交详情
$ cnb-rs user activity-detail --type commit --repo org/repo --date 202501

# 查看指定用户的 PR 活动
$ cnb-rs user activity-detail zhangsan --type pr --repo org/repo --date 20250115

# JSON 输出
$ cnb-rs user activity-detail --type review --repo org/repo --date 202501 --json
```

## 另请参阅

- [cnb-rs user](/user/)
- [cnb-rs user activities](/user/activities)
