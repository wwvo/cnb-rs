# cnb user following

```
cnb user following [<username>] [options]
```

查看指定用户的关注列表，不指定则查看当前用户。

## 参数

| 参数            | 缩写 | 说明                           |
|-----------------|------|--------------------------------|
| `[<username>]`  |      | 用户名（不指定则查看当前用户） |
| `--limit <n>`   | `-L` | 最大数量（默认 100）           |

## 示例

```bash
cnb user following
cnb user following zhangsan
cnb user following --json
```
