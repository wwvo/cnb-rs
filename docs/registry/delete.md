# cnb registry delete

```
cnb registry delete <registry> [options]
```

删除制品库。

## 参数

| 参数          | 缩写 | 说明             |
|---------------|------|------------------|
| `<registry>`  |      | 制品库路径       |
| `--yes`       | `-y` | 跳过确认提示     |

## 示例

```bash
cnb registry delete my-org/my-registry
cnb registry delete my-org/my-registry --yes
```
