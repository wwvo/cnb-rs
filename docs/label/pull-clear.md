# cnb label pull-clear

```
cnb label pull-clear <number> [-y]
```

清空指定 Pull Request 的所有标签。默认需要交互确认。

## 参数

| 参数       | 缩写 | 说明           |
|------------|------|----------------|
| `<number>` |      | Pull 编号      |
| `--yes`    | `-y` | 跳过确认提示   |

## 示例

```bash
cnb label pull-clear 10
# ⚠ 确认清空 Pull #10 的所有标签？(y/N) y
# ✓ 已清空 Pull #10 的所有标签

cnb label pull-clear 10 -y
# ✓ 已清空 Pull #10 的所有标签
```

## 另请参阅

- [cnb label pull-list](/label/pull-list)
- [cnb label pull-set](/label/pull-set)
