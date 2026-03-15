# cnb-rs registry package detail

```
cnb-rs registry package detail <type> <name> --registry <registry>
```

查看制品详情。

返回制品的详细信息，包括名称、类型、大小、标签数、创建时间等。

## 选项

- `<type>`: 制品类型（必填）
- `<name>`: 制品名称（必填）
- `-r, --registry <PATH>`: 制品库路径（必填）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看 Docker 制品详情
$ cnb-rs registry package detail docker my-app --registry my-org/my-registry

# 查看 npm 制品详情
$ cnb-rs registry package detail npm my-lib --registry my-org/npm-registry
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry package list](/registry/package-list)
- [cnb-rs registry package delete](/registry/package-delete)
