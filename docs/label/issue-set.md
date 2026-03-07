# cnb label issue-set

```
cnb label issue-set <number> -l <labels>
```

替换指定 Issue 的所有标签（现有标签将被完全替换）。

## 参数

| 参数              | 缩写 | 说明           |
|-------------------|------|----------------|
| `<number>`        |      | Issue 编号     |
| `--labels <list>` | `-l` | 新标签列表     |

## 示例

```bash
cnb label issue-set 42 -l "wontfix"
# ✓ 已替换 Issue #42 的标签为: wontfix
```

## 另请参阅

- [cnb label issue-list](/label/issue-list)
- [cnb label issue-clear](/label/issue-clear)
