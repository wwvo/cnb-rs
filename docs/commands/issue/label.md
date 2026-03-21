---
title: cnb-rs issue label
---

# cnb-rs issue label

```
cnb-rs issue label <subcommand>
```

Issue 标签管理，支持列出、添加、替换、删除和清空标签。

标签用于对 Issue 进行分类和标记，便于过滤和管理。

## 子命令

### label list

```
cnb-rs issue label list <NUMBER>
```

列出指定 Issue 的所有标签。

- `<NUMBER>`: Issue 编号（必填）

### label add

```
cnb-rs issue label add <NUMBER> <LABELS>
```

为指定 Issue 添加标签。不影响已有标签，自动去重去空。

- `<NUMBER>`: Issue 编号（必填）
- `<LABELS>`: 标签名称，多个用逗号分隔（必填）

### label set

```
cnb-rs issue label set <NUMBER> <LABELS>
```

替换指定 Issue 的所有标签。会移除所有现有标签，设置为新的标签列表。

- `<NUMBER>`: Issue 编号（必填）
- `<LABELS>`: 新的标签列表，多个用逗号分隔（必填）

### label remove

```
cnb-rs issue label remove <NUMBER> <NAME>
```

删除指定 Issue 的某个标签。

- `<NUMBER>`: Issue 编号（必填）
- `<NAME>`: 要删除的标签名称（必填）

### label clear

```
cnb-rs issue label clear <NUMBER>
```

清空指定 Issue 的所有标签。

- `<NUMBER>`: Issue 编号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出（适用于 list）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出标签
$ cnb-rs issue label list 42

# 添加标签
$ cnb-rs issue label add 42 "bug,enhancement,P0"
✓ Issue #42 标签已添加

# 替换所有标签
$ cnb-rs issue label set 42 "bug,P1"
✓ Issue #42 标签已替换

# 删除指定标签
$ cnb-rs issue label remove 42 bug
✓ Issue #42 标签 'bug' 已删除

# 清空所有标签
$ cnb-rs issue label clear 42
✓ Issue #42 所有标签已清空
```

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs label](/label/)
