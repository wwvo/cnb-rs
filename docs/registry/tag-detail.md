# cnb registry tag detail

```
cnb registry tag detail <type> <name> <tag> --registry <registry> [options]
```

查看制品标签详情。

## 参数

| 参数                | 缩写 | 说明                           |
|---------------------|------|--------------------------------|
| `<type>`            |      | 制品类型                       |
| `<name>`            |      | 制品名称                       |
| `<tag>`             |      | 标签名                         |
| `--registry <path>` | `-r` | 制品库路径                     |
| `--arch <arch>`     |      | 架构（Docker 制品时指定）      |

## 示例

```bash
cnb registry tag detail docker my-app latest --registry my-org/my-registry --arch linux/amd64
cnb registry tag detail helm my-chart 1.0.0 --registry my-org/helm-charts
cnb registry tag detail npm my-lib 1.2.3 --registry my-org/npm-registry
```
