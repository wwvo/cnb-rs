---
title: cnb-rs group transfer
---

# cnb-rs group transfer

```
cnb-rs group transfer <GROUP> --target <TARGET> [options]
```

将组织转移到另一个父组织下。**此操作需要交互确认**。

## 选项

- `<GROUP>`: 要转移的组织路径（必填）
- `-t, --target <TARGET>`: 目标父组织路径（必填）
- `--confirm`: 跳过交互确认

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 转移组织（需确认）
$ cnb-rs group transfer child-org --target parent-org

# 跳过确认
$ cnb-rs group transfer child-org --target parent-org --confirm
```

## 另请参阅

- [cnb-rs group](/group/)
