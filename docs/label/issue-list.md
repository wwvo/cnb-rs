# cnb label issue-list

```
cnb label issue-list <number>
```

列出指定 Issue 的所有标签。

## 参数

| 参数       | 说明       |
|------------|------------|
| `<number>` | Issue 编号 |

## 示例

```bash
cnb label issue-list 42
# bug
# enhancement

cnb label issue-list 42 --json
```

## 另请参阅

- [cnb label issue-add](/label/issue-add)
- [cnb label issue-remove](/label/issue-remove)
