# cnb label issue-remove

```
cnb label issue-remove <number> <name>
```

移除指定 Issue 的单个标签。

## 参数

| 参数       | 说明       |
|------------|------------|
| `<number>` | Issue 编号 |
| `<name>`   | 标签名称   |

## 示例

```bash
cnb label issue-remove 42 "bug"
# ✓ 已从 Issue #42 移除标签: bug
```

## 另请参阅

- [cnb label issue-list](/label/issue-list)
- [cnb label issue-clear](/label/issue-clear)
