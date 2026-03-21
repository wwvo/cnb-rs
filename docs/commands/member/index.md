---
title: cnb-rs member
---

# cnb-rs member

```
cnb-rs member <subcommand>
```

管理仓库成员、组织成员和外部贡献者。

支持对仓库和组织维度的成员进行增删改查，以及权限层级的查看。
成员权限通过继承机制传递：组织成员自动继承到子仓库，可通过 `inherited` 命令查看继承关系。

## 可用命令

### 仓库成员

- [cnb-rs member repo-list](/member/repo-list) — 列出仓库直接成员
- [cnb-rs member repo-add](/member/repo-add) — 添加仓库成员
- [cnb-rs member repo-update](/member/repo-update) — 更新仓库成员权限
- [cnb-rs member repo-remove](/member/repo-remove) — 移除仓库成员
- [cnb-rs member repo-access-level](/member/repo-access-level) — 查看自己在仓库的权限
- [cnb-rs member repo-user-access](/member/repo-user-access) — 查看指定成员在仓库的权限层级
- [cnb-rs member repo-inherited](/member/repo-inherited) — 列出仓库继承成员
- [cnb-rs member repo-all](/member/repo-all) — 列出仓库所有有效成员

### 组织成员

- [cnb-rs member group-list](/member/group-list) — 列出组织直接成员
- [cnb-rs member group-add](/member/group-add) — 添加组织成员
- [cnb-rs member group-update](/member/group-update) — 更新组织成员权限
- [cnb-rs member group-remove](/member/group-remove) — 移除组织成员
- [cnb-rs member group-access-level](/member/group-access-level) — 查看自己在组织的权限
- [cnb-rs member group-user-access](/member/group-user-access) — 查看指定成员在组织的权限层级
- [cnb-rs member group-inherited](/member/group-inherited) — 列出组织继承成员

### 外部贡献者

- [cnb-rs member collaborator-list](/member/collaborator-list) — 列出外部贡献者
- [cnb-rs member collaborator-update](/member/collaborator-update) — 更新外部贡献者权限
- [cnb-rs member collaborator-remove](/member/collaborator-remove) — 移除外部贡献者

## 权限等级

| 等级      | 说明   |
| --------- | ------ |
| Guest     | 访客   |
| Reporter  | 报告者 |
| Developer | 开发者 |
| Master    | 管理者 |
| Owner     | 拥有者 |

## 示例

```bash
# 列出仓库成员
$ cnb-rs member repo-list

# 添加仓库成员
$ cnb-rs member repo-add zhangsan --role Developer

# 列出组织成员
$ cnb-rs member group-list --group myorg

# 列出外部贡献者
$ cnb-rs member collaborator-list --group myorg
```

## 另请参阅

- [cnb-rs](/cnb)
- [cnb-rs repo](/repo/)
- [cnb-rs group](/group/)
