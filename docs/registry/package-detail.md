# cnb registry package detail

```
cnb registry package detail <type> <name> --registry <registry>
```

查看制品详情。

## 参数

| 参数                | 缩写 | 说明         |
|---------------------|------|--------------|
| `<type>`            |      | 制品类型     |
| `<name>`            |      | 制品名称     |
| `--registry <path>` | `-r` | 制品库路径   |

## 示例

```bash
cnb registry package detail docker my-app --registry my-org/my-registry
cnb registry package detail npm my-lib --registry my-org/npm-registry
```
