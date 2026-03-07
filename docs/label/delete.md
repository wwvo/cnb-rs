# cnb label delete

```
cnb label delete <name> [-y]
```

删除仓库标签。默认需要交互确认。

## 参数

| 参数     | 缩写 | 说明           |
|----------|------|----------------|
| `<name>` |      | 标签名称       |
| `--yes`  | `-y` | 跳过确认提示   |

## 示例

```bash
cnb label delete "bug"
# ⚠ 确认删除标签 bug？(y/N) y
# ✓ 标签 bug 已删除

cnb label delete "bug" -y
# ✓ 标签 bug 已删除
```

## 另请参阅

- [cnb label list](/label/list)
- [cnb label create](/label/create)
