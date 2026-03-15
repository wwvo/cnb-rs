# cnb-rs registry tag list

```
cnb-rs registry tag list <type> <name> --registry <registry> [options]
```

列出制品标签。

输出为表格格式，包含标签名、大小、创建时间等信息。

## 选项

- `<type>`: 制品类型（必填）
- `<name>`: 制品名称（必填）
- `-r, --registry <PATH>`: 制品库路径（必填）
- `--tag-name <KEYWORD>`: 按标签名搜索
- `--order <ORDERING>`: 排序方式

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出制品标签
$ cnb-rs registry tag list docker my-app --registry my-org/my-registry

# 搜索标签
$ cnb-rs registry tag list npm my-lib --registry my-org/npm-registry --tag-name "1.0"
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry tag detail](/registry/tag-detail)
- [cnb-rs registry tag delete](/registry/tag-delete)
