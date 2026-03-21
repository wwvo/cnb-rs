---
title: cnb-rs group member
---

# cnb-rs group member

```
cnb-rs group member <subcommand>
```

组织成员管理。

包含成员列表、添加、更新、移除、权限查看五个子命令。

## 子命令

### member list

```
cnb-rs group member list <GROUP> [options]
```

列出组织的直接成员。

- `<GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 按角色过滤
- `-s, --search <KEYWORD>`: 关键字搜索
- `--inherited`: 列出继承成员（来自父组织）

### member add

```
cnb-rs group member add <GROUP> <USERNAME> [options]
```

添加组织成员。

- `<GROUP>`: 组织路径（必填）
- `<USERNAME>`: 要添加的用户名（必填）
- `-r, --role <ROLE>`: 权限级别（默认：`Developer`），可选值：`Guest`、`Reporter`、`Developer`、`Master`、`Owner`

### member update

```
cnb-rs group member update <GROUP> <USERNAME> --role <ROLE>
```

更新成员权限。

- `<GROUP>`: 组织路径（必填）
- `<USERNAME>`: 用户名（必填）
- `-r, --role <ROLE>`: 新的权限级别（必填），可选值：`Guest`、`Reporter`、`Developer`、`Master`、`Owner`

### member remove

```
cnb-rs group member remove <GROUP> <USERNAME> [options]
```

移除组织成员。需要交互确认。

- `<GROUP>`: 组织路径（必填）
- `<USERNAME>`: 要移除的用户名（必填）
- `--confirm`: 跳过交互确认

### member access-level

```
cnb-rs group member access-level <GROUP> [USERNAME]
```

查看成员权限级别。不指定用户名则查看当前用户权限。

- `<GROUP>`: 组织路径（必填）
- `[USERNAME]`: 用户名（可选，不指定则查看当前用户权限）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出（适用于 list、access-level）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出成员
$ cnb-rs group member list my-org

# 按角色过滤
$ cnb-rs group member list my-org --role Owner

# 列出继承成员
$ cnb-rs group member list my-org/sub-team --inherited

# 添加成员
$ cnb-rs group member add my-org alice --role Developer

# 更新权限
$ cnb-rs group member update my-org alice --role Master

# 移除成员
$ cnb-rs group member remove my-org alice

# 查看我的权限
$ cnb-rs group member access-level my-org

# 查看指定用户权限
$ cnb-rs group member access-level my-org alice

# JSON 格式输出
$ cnb-rs group member list my-org --json
```

## 另请参阅

- [cnb-rs group](/group/)
- [cnb-rs member](/member/)
