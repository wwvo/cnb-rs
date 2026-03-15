# cnb-rs label update

```
cnb-rs label update <name> [--new-name <name>] [-c <color>] [-d <description>]
```

更新仓库标签的名称、颜色或描述。

## 选项

- `<name>`: 当前标签名称（必填）
- `--new-name <NAME>`: 新名称
- `-c, --color <HEX>`: 新颜色（十六进制格式，不含 `#`）
- `-d, --description <DESC>`: 新描述

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs label update "bug" --color "ff0000" -d "严重 Bug"
✓ 标签 bug 已更新

$ cnb-rs label update "bug" --new-name "critical-bug"
✓ 标签 bug 已重命名为 critical-bug
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label list](/label/list)
- [cnb-rs label delete](/label/delete)
