# cnb label list

```
cnb label list [--keyword <text>]
```

列出仓库所有标签，支持关键字搜索。

## 参数

| 参数               | 缩写 | 说明       |
|--------------------|------|------------|
| `--keyword <text>` | `-k` | 搜索关键字 |

## 示例

```bash
# 列出所有标签
cnb label list

# 搜索标签
cnb label list -k "bug"

# JSON 输出
cnb label list --json
```

## 另请参阅

- [cnb label](/label/)
- [cnb label create](/label/create)
