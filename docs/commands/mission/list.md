---
title: cnb-rs mission list
---

# cnb-rs mission list

```
cnb-rs mission list --group <group> [options]
```

列出组织下的任务集。

输出为表格格式，包含任务集名称、路径、可见性、关联仓库数等信息。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `-s, --search <KEYWORD>`: 搜索关键字
- `--order-by <FIELD>`: 排序字段
- `--desc`: 降序排列

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出组织下的任务集
$ cnb-rs mission list --group my-org

# 搜索任务集
$ cnb-rs mission list --group my-org --search sprint

# 按创建时间降序
$ cnb-rs mission list --group my-org --order-by created_at --desc

# JSON 格式输出
$ cnb-rs mission list --group my-org --json
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission create](/mission/create)
