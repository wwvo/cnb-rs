---
title: cnb-rs build delete-log
---

# cnb-rs build delete-log

```
cnb-rs build delete-log <sn> [-y]
```

删除指定构建的日志。

执行前会要求确认，可通过 `--yes` 跳过确认提示。

## 选项

- `<sn>`: 构建号（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除构建日志（需确认）
$ cnb-rs build delete-log cnb-1qa-1i3f5ecau
确认删除构建 cnb-1qa-1i3f5ecau 的日志？(y/N) y
✓ 构建日志已删除

# 跳过确认
$ cnb-rs build delete-log cnb-1qa-1i3f5ecau -y
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build list](/build/list)
- [cnb-rs build download-log](/build/download-log)
