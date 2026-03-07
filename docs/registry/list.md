# cnb registry list

```
cnb registry list --group <group> [options]
```

列出组织下的制品库。

## 参数

| 参数                 | 缩写 | 说明             |
|----------------------|------|------------------|
| `--group <group>`    | `-g` | 组织路径         |
| `--type <type>`      | `-t` | 制品库类型过滤   |
| `--search <keyword>` | `-s` | 搜索关键字       |
| `--order-by <field>` |      | 排序字段         |
| `--desc`             |      | 降序排列         |

## 示例

```bash
cnb registry list --group my-org
cnb registry list --group my-org --type docker
cnb registry list --group my-org --json
```
