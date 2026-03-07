# cnb registry package list

```
cnb registry package list <type> --registry <registry> [options]
```

列出制品。

## 参数

| 参数                   | 缩写 | 说明             |
|------------------------|------|------------------|
| `<type>`               |      | 制品类型         |
| `--registry <path>`    | `-r` | 制品库路径       |
| `--name <keyword>`     | `-n` | 按名称搜索       |
| `--order <ordering>`   |      | 排序方式         |

## 示例

```bash
cnb registry package list docker --registry my-org/my-registry
cnb registry package list npm --registry my-org/npm-registry --name my-lib
cnb registry package list docker --registry my-org/my-registry --json
```
