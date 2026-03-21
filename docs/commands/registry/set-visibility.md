---
title: cnb-rs registry set-visibility
---

# cnb-rs registry set-visibility

```
cnb-rs registry set-visibility <registry> <visibility>
```

设置制品库可见性。

## 选项

- `<registry>`: 制品库路径，格式 `group/registry`（必填）
- `<visibility>`: 可见性：`public`、`private`、`secret`（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 设为私有
$ cnb-rs registry set-visibility my-org/my-registry private

# 设为公开
$ cnb-rs registry set-visibility my-org/my-registry public
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry list](/registry/list)
