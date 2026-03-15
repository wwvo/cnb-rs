# cnb-rs mission view list

```
cnb-rs mission view list <mission>
```

列出任务集的视图。

输出为表格格式，包含视图 ID、名称和类型。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs mission view list my-org/sprint-2025-q1

# JSON 格式输出
$ cnb-rs mission view list my-org/sprint-2025-q1 --json
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission view get](/mission/view-get)
- [cnb-rs mission view add](/mission/view-add)
