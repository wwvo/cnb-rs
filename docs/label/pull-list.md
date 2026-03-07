# cnb label pull-list

```
cnb label pull-list <number>
```

列出指定 Pull Request 的所有标签。

## 参数

| 参数       | 说明      |
|------------|-----------|
| `<number>` | Pull 编号 |

## 示例

```bash
cnb label pull-list 10
# feature
# review-needed

cnb label pull-list 10 --json
```

## 另请参阅

- [cnb label pull-add](/label/pull-add)
- [cnb label pull-remove](/label/pull-remove)
