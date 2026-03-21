---
title: cnb-rs issue reopen
---

# cnb-rs issue reopen

```
cnb-rs issue reopen <NUMBER>
```

重新打开已关闭的 Issue。

将 Issue 状态设置为 `open`，关闭原因设置为 `reopened`。

## 选项

- `<NUMBER>`: Issue 编号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 重新打开 Issue
$ cnb-rs issue reopen 42
✓ Issue #42 已重新打开

# 指定仓库
$ cnb-rs --repo org/repo issue reopen 42
✓ Issue #42 已重新打开
```

## API

| 步骤           | API                                     | 方法  | 说明            |
| -------------- | --------------------------------------- | ----- | --------------- |
| 重新打开 Issue | `${API}/repos/{repo}/-/issues/{number}` | PATCH | 更新 Issue 状态 |

**请求体：**

```json
{
  "state": "open",
  "state_reason": "reopened"
}
```

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue close](/issue/close)
- [cnb-rs issue view](/issue/view)
