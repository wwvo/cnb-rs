---
title: cnb-rs member repo-access-level
---

# cnb-rs member repo-access-level

```
cnb-rs member repo-access-level [options]
```

查看自己在仓库的权限。

显示当前认证用户在指定仓库中的权限等级，可通过 `--include-inherit` 同时显示继承权限。

## 选项

- `--include-inherit`: 包含继承权限

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看自己在仓库的权限
$ cnb-rs member repo-access-level

# 包含继承权限
$ cnb-rs member repo-access-level --include-inherit

# JSON 格式输出
$ cnb-rs member repo-access-level --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-user-access](/member/repo-user-access)
