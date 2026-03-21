---
title: cnb-rs member group-inherited
---

# cnb-rs member group-inherited

```
cnb-rs member group-inherited --group <group> [options]
```

列出组织继承成员。

继承成员是通过上级组织权限自动获得当前组织访问权限的用户。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 按角色过滤
- `-s, --search <KEYWORD>`: 搜索成员

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出组织继承成员
$ cnb-rs member group-inherited --group myorg

# 按角色过滤
$ cnb-rs member group-inherited --group myorg --role Master

# JSON 格式输出
$ cnb-rs member group-inherited --group myorg --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member group-list](/member/group-list)
