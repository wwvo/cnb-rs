# cnb label pull-remove

```
cnb label pull-remove <number> <name>
```

移除指定 Pull Request 的单个标签。

## 参数

| 参数       | 说明      |
|------------|-----------|
| `<number>` | Pull 编号 |
| `<name>`   | 标签名称  |

## 示例

```bash
cnb label pull-remove 10 "wip"
# ✓ 已从 Pull #10 移除标签: wip
```

## 另请参阅

- [cnb label pull-list](/label/pull-list)
- [cnb label pull-clear](/label/pull-clear)
