# cnb label pull-set

```
cnb label pull-set <number> -l <labels>
```

替换指定 Pull Request 的所有标签（现有标签将被完全替换）。

## 参数

| 参数              | 缩写 | 说明         |
|-------------------|------|--------------|
| `<number>`        |      | Pull 编号    |
| `--labels <list>` | `-l` | 新标签列表   |

## 示例

```bash
cnb label pull-set 10 -l "wip"
# ✓ 已替换 Pull #10 的标签为: wip
```

## 另请参阅

- [cnb label pull-list](/label/pull-list)
- [cnb label pull-clear](/label/pull-clear)
