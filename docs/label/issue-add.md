# cnb label issue-add

```
cnb label issue-add <number> -l <labels>
```

为指定 Issue 添加标签。

## 参数

| 参数              | 缩写 | 说明                           |
|-------------------|------|--------------------------------|
| `<number>`        |      | Issue 编号                     |
| `--labels <list>` | `-l` | 标签名称（逗号分隔或多次指定） |

## 示例

```bash
cnb label issue-add 42 -l "bug" -l "urgent"
# ✓ 已为 Issue #42 添加标签: bug, urgent

cnb label issue-add 42 -l "bug,urgent"
# ✓ 已为 Issue #42 添加标签: bug, urgent
```

## 另请参阅

- [cnb label issue-list](/label/issue-list)
- [cnb label issue-remove](/label/issue-remove)
