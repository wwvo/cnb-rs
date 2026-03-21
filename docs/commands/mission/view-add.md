---
title: cnb-rs mission view add
---

# cnb-rs mission view add

```
cnb-rs mission view add <mission> --name <name> --type <type> [--id <id>]
```

添加或修改任务集视图。

不指定 `--id` 时创建新视图，指定 `--id` 时修改已有视图。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）
- `--name <NAME>`: 视图名称（必填）
- `--type <TYPE>`: 视图类型（必填）
- `--id <ID>`: 视图 ID（修改已有视图时指定）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 添加新视图
$ cnb-rs mission view add my-org/sprint-2025-q1 --name "看板视图" --type kanban

# 修改已有视图
$ cnb-rs mission view add my-org/sprint-2025-q1 --id view-001 --name "重命名看板" --type kanban
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission view list](/mission/view-list)
- [cnb-rs mission view get](/mission/view-get)
