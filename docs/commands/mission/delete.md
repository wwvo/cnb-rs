---
title: cnb-rs mission delete
---

# cnb-rs mission delete

```
cnb-rs mission delete <mission> [options]
```

删除任务集。

执行前会要求确认，可通过 `--yes` 跳过确认提示。删除操作不可撤销。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除任务集（需确认）
$ cnb-rs mission delete my-org/sprint-2025-q1

# 跳过确认
$ cnb-rs mission delete my-org/sprint-2025-q1 --yes
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission list](/mission/list)
- [cnb-rs mission create](/mission/create)
