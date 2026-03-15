# cnb-rs mission set-visibility

```
cnb-rs mission set-visibility <mission> <visibility>
```

设置任务集可见性。

## 选项

- `<mission>`: 任务集路径，格式 `group/mission`（必填）
- `<visibility>`: 可见性：`public`、`private`、`secret`（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 设为私有
$ cnb-rs mission set-visibility my-org/sprint-2025-q1 private
✓ 可见性已更新为 private

# 设为公开
$ cnb-rs mission set-visibility my-org/sprint-2025-q1 public
```

## 另请参阅

- [cnb-rs mission](/mission/)
- [cnb-rs mission list](/mission/list)
