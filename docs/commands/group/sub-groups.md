---
title: cnb-rs group sub-groups
---

# cnb-rs group sub-groups

```
cnb-rs group sub-groups <GROUP> [options]
```

列出指定组织下的子组织。

与 `cnb-rs group list --group` 不同，此命令调用的是子组织列表 API，不受权限过滤。

## 选项

- `<GROUP>`: 父组织路径（必填）
- `-s, --search <KEYWORD>`: 关键字过滤

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出子组织
$ cnb-rs group sub-groups my-org

# 搜索子组织
$ cnb-rs group sub-groups my-org --search dev

# JSON 格式输出
$ cnb-rs group sub-groups my-org --json
```

## 另请参阅

- [cnb-rs group](/group/)
- [cnb-rs group list](/group/list)
