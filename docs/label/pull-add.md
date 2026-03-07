# cnb label pull-add

```
cnb label pull-add <number> -l <labels>
```

为指定 Pull Request 添加标签。

## 参数

| 参数              | 缩写 | 说明                           |
|-------------------|------|--------------------------------|
| `<number>`        |      | Pull 编号                      |
| `--labels <list>` | `-l` | 标签名称（逗号分隔或多次指定） |

## 示例

```bash
cnb label pull-add 10 -l "approved" -l "ready-to-merge"
# ✓ 已为 Pull #10 添加标签: approved, ready-to-merge
```

## 另请参阅

- [cnb label pull-list](/label/pull-list)
- [cnb label pull-remove](/label/pull-remove)
