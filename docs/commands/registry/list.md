# cnb-rs registry list

```
cnb-rs registry list --group <group> [options]
```

列出组织下的制品库。

输出为表格格式，包含制品库名称、类型、可见性、已用空间等信息。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `-t, --type <TYPE>`: 制品库类型过滤（如 `docker`、`npm`）
- `-s, --search <KEYWORD>`: 搜索关键字
- `--order-by <FIELD>`: 排序字段
- `--desc`: 降序排列

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出组织下的制品库
$ cnb-rs registry list --group my-org

# 按类型过滤
$ cnb-rs registry list --group my-org --type docker

# JSON 格式输出
$ cnb-rs registry list --group my-org --json
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry package list](/registry/package-list)
