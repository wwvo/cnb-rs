---
title: cnb-rs registry delete
---

# cnb-rs registry delete

```
cnb-rs registry delete <registry> [options]
```

删除制品库。

执行前会要求确认，可通过 `--yes` 跳过确认提示。删除操作不可撤销。

## 选项

- `<registry>`: 制品库路径，格式 `group/registry`（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除制品库（需确认）
$ cnb-rs registry delete my-org/my-registry

# 跳过确认
$ cnb-rs registry delete my-org/my-registry --yes
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry list](/registry/list)
