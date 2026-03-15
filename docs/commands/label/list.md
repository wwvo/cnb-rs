# cnb-rs label list

```
cnb-rs label list [--keyword <text>]
```

列出仓库所有标签，支持关键字搜索。

## 选项

- `-k, --keyword <TEXT>`: 搜索关键字

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出所有标签
$ cnb-rs label list

# 搜索标签
$ cnb-rs label list -k "bug"

# JSON 格式输出
$ cnb-rs label list --json
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label create](/label/create)
