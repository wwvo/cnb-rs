---
title: cnb-rs label delete
---

# cnb-rs label delete

```
cnb-rs label delete <name> [-y]
```

删除仓库标签。

执行前会要求确认，可通过 `--yes` 跳过确认提示。

## 选项

- `<name>`: 标签名称（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除标签（需确认）
$ cnb-rs label delete "bug"
确认删除标签 bug？(y/N) y
✓ 标签 bug 已删除

# 跳过确认
$ cnb-rs label delete "bug" -y
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label list](/label/list)
- [cnb-rs label create](/label/create)
