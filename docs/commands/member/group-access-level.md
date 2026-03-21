---
title: cnb-rs member group-access-level
---

# cnb-rs member group-access-level

```
cnb-rs member group-access-level --group <group> [options]
```

查看自己在组织的权限。

显示当前认证用户在指定组织中的权限等级，可通过 `--include-inherit` 同时显示继承权限。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `--include-inherit`: 包含继承权限

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看自己在组织的权限
$ cnb-rs member group-access-level --group myorg

# JSON 格式输出
$ cnb-rs member group-access-level --group myorg --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member group-user-access](/member/group-user-access)
