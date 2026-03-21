---
title: cnb-rs label create
---

# cnb-rs label create

```
cnb-rs label create -n <name> -c <color> [-d <description>]
```

创建仓库标签。

## 选项

- `-n, --name <NAME>`: 标签名称（必填）
- `-c, --color <HEX>`: 颜色，十六进制格式，不含 `#`（必填）
- `-d, --description <DESC>`: 标签描述

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs label create -n "bug" -c "d73a4a" -d "Bug 修复"
✓ 标签 bug 已创建

$ cnb-rs label create -n "feature" -c "0075ca"
✓ 标签 feature 已创建
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label list](/label/list)
- [cnb-rs label update](/label/update)
