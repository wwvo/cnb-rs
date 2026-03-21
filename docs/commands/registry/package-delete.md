---
title: cnb-rs registry package delete
---

# cnb-rs registry package delete

```
cnb-rs registry package delete <type> <name> --registry <registry> [options]
```

删除制品。

执行前会要求确认，可通过 `--yes` 跳过确认提示。删除操作不可撤销。

## 选项

- `<type>`: 制品类型（必填）
- `<name>`: 制品名称（必填）
- `-r, --registry <PATH>`: 制品库路径（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除制品（需确认）
$ cnb-rs registry package delete docker my-app --registry my-org/my-registry

# 跳过确认
$ cnb-rs registry package delete docker my-app --registry my-org/my-registry --yes
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry package list](/registry/package-list)
