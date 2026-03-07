# cnb label update

```
cnb label update <name> [--new-name <name>] [-c <color>] [-d <description>]
```

更新仓库标签的名称、颜色或描述。

## 参数

| 参数                   | 缩写 | 必填 | 说明         |
|------------------------|------|------|--------------|
| `<name>`               |      | 是   | 当前标签名称 |
| `--new-name <name>`    |      | 否   | 新名称       |
| `--color <hex>`        | `-c` | 否   | 新颜色       |
| `--description <desc>` | `-d` | 否   | 新描述       |

## 示例

```bash
cnb label update "bug" --color "ff0000" -d "严重 Bug"
# ✓ 标签 bug 已更新

cnb label update "bug" --new-name "critical-bug"
# ✓ 标签 bug 已重命名为 critical-bug
```

## 另请参阅

- [cnb label list](/label/list)
- [cnb label delete](/label/delete)
