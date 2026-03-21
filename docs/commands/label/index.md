---
title: cnb-rs label
---

# cnb-rs label

```
cnb-rs label <subcommand>
```

管理仓库标签，以及 Issue / Pull Request 上的标签操作。

标签用于对 Issue 和 Pull Request 进行分类和标记，支持自定义名称、颜色和描述。

## 可用命令

### 仓库标签

- [cnb-rs label list](/label/list) — 列出仓库所有标签
- [cnb-rs label create](/label/create) — 创建标签
- [cnb-rs label update](/label/update) — 更新标签
- [cnb-rs label delete](/label/delete) — 删除标签

### Issue 标签

- [cnb-rs label issue-list](/label/issue-list) — 列出 Issue 标签
- [cnb-rs label issue-add](/label/issue-add) — 为 Issue 添加标签
- [cnb-rs label issue-set](/label/issue-set) — 替换 Issue 标签
- [cnb-rs label issue-remove](/label/issue-remove) — 移除 Issue 单个标签
- [cnb-rs label issue-clear](/label/issue-clear) — 清空 Issue 所有标签

### Pull 标签

- [cnb-rs label pull-list](/label/pull-list) — 列出 Pull 标签
- [cnb-rs label pull-add](/label/pull-add) — 为 Pull 添加标签
- [cnb-rs label pull-set](/label/pull-set) — 替换 Pull 标签
- [cnb-rs label pull-remove](/label/pull-remove) — 移除 Pull 单个标签
- [cnb-rs label pull-clear](/label/pull-clear) — 清空 Pull 所有标签

## 示例

```bash
# 列出仓库所有标签
$ cnb-rs label list

# 创建标签
$ cnb-rs label create -n "bug" -c "d73a4a" -d "Bug 修复"

# 为 Issue 添加标签
$ cnb-rs label issue-add 42 -l "bug" -l "urgent"

# 列出 Pull 标签
$ cnb-rs label pull-list 10
```

## 另请参阅

- [cnb-rs](/cnb)
- [cnb-rs issue](/issue/)
- [cnb-rs pr](/pr/)
