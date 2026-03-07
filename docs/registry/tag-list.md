# cnb registry tag list

```
cnb registry tag list <type> <name> --registry <registry> [options]
```

列出制品标签。

## 参数

| 参数                    | 缩写 | 说明             |
|-------------------------|------|------------------|
| `<type>`                |      | 制品类型         |
| `<name>`                |      | 制品名称         |
| `--registry <path>`     | `-r` | 制品库路径       |
| `--tag-name <keyword>`  |      | 按标签名搜索     |
| `--order <ordering>`    |      | 排序方式         |

## 示例

```bash
cnb registry tag list docker my-app --registry my-org/my-registry
cnb registry tag list npm my-lib --registry my-org/npm-registry --tag-name "1.0"
```
