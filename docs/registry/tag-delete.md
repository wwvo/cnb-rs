# cnb registry tag delete

```
cnb registry tag delete <type> <name> <tag> --registry <registry> [options]
```

删除制品标签。

## 参数

| 参数                | 缩写 | 说明             |
|---------------------|------|------------------|
| `<type>`            |      | 制品类型         |
| `<name>`            |      | 制品名称         |
| `<tag>`             |      | 标签名           |
| `--registry <path>` | `-r` | 制品库路径       |
| `--yes`             | `-y` | 跳过确认提示     |

## 示例

```bash
cnb registry tag delete docker my-app v1.0.0 --registry my-org/my-registry
cnb registry tag delete docker my-app v1.0.0 --registry my-org/my-registry --yes
```
