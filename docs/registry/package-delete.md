# cnb registry package delete

```
cnb registry package delete <type> <name> --registry <registry> [options]
```

删除制品。

## 参数

| 参数                | 缩写 | 说明             |
|---------------------|------|------------------|
| `<type>`            |      | 制品类型         |
| `<name>`            |      | 制品名称         |
| `--registry <path>` | `-r` | 制品库路径       |
| `--yes`             | `-y` | 跳过确认提示     |

## 示例

```bash
cnb registry package delete docker my-app --registry my-org/my-registry
cnb registry package delete docker my-app --registry my-org/my-registry --yes
```
