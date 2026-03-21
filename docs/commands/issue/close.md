---
title: cnb-rs issue close
---

# cnb-rs issue close

```
cnb-rs issue close <NUMBER> [flags]
```

关闭指定编号的 Issue。

将 Issue 状态设置为 `closed`，支持通过 `--reason` 指定关闭原因。

## 选项

- `<NUMBER>`: Issue 编号（必填）
- `-r, --reason <REASON>`: 关闭原因，可选值：`completed`（已完成）、`not-planned`（不计划处理）（默认：`completed`）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 关闭 Issue（默认原因：已完成）
$ cnb-rs issue close 123
✓ Issue #123 已关闭（原因: completed）

# 以"不计划处理"关闭
$ cnb-rs issue close 123 -r not-planned
✓ Issue #123 已关闭（原因: not_planned）
```

## API

| 步骤       | API                                     | 方法  | 说明            |
| ---------- | --------------------------------------- | ----- | --------------- |
| 关闭 Issue | `${API}/repos/{repo}/-/issues/{number}` | PATCH | 更新 Issue 状态 |

**请求体：**

```json
{
  "state": "closed",
  "state_reason": "completed"
}
```

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue reopen](/issue/reopen)
