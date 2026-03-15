# cnb-rs registry package list

```
cnb-rs registry package list <type> --registry <registry> [options]
```

列出制品。

输出为表格格式，包含制品名称、标签数、大小等信息。

## 选项

- `<type>`: 制品类型，如 `docker`、`npm`、`helm`（必填）
- `-r, --registry <PATH>`: 制品库路径（必填）
- `-n, --name <KEYWORD>`: 按名称搜索
- `--order <ORDERING>`: 排序方式

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出 Docker 制品
$ cnb-rs registry package list docker --registry my-org/my-registry

# 搜索 npm 制品
$ cnb-rs registry package list npm --registry my-org/npm-registry --name my-lib

# JSON 格式输出
$ cnb-rs registry package list docker --registry my-org/my-registry --json
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry package detail](/registry/package-detail)
- [cnb-rs registry package delete](/registry/package-delete)
