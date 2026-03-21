---
title: cnb-rs mission view sort
---

# cnb-rs mission view sort

```
cnb-rs mission view sort <mission> --ids <id1,id2,id3>
```

排序任务集视图。

按指定顺序重新排列视图，传入的 ID 列表顺序即为最终显示顺序。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）
- `--ids <IDS>`: 视图 ID 列表，按期望顺序排列（逗号分隔，必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs mission view sort my-org/sprint-2025-q1 --ids "view-002,view-001,view-003"
✓ 视图排序已更新
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission view list](/mission/view-list)
