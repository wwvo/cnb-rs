# cnb label issue-clear

```
cnb label issue-clear <number> [-y]
```

清空指定 Issue 的所有标签。默认需要交互确认。

## 参数

| 参数       | 缩写 | 说明           |
|------------|------|----------------|
| `<number>` |      | Issue 编号     |
| `--yes`    | `-y` | 跳过确认提示   |

## 示例

```bash
cnb label issue-clear 42
# ⚠ 确认清空 Issue #42 的所有标签？(y/N) y
# ✓ 已清空 Issue #42 的所有标签

cnb label issue-clear 42 -y
# ✓ 已清空 Issue #42 的所有标签
```

## 另请参阅

- [cnb label issue-list](/label/issue-list)
- [cnb label issue-set](/label/issue-set)
