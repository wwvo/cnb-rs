# cnb-rs registry tag detail

```
cnb-rs registry tag detail <type> <name> <tag> --registry <registry> [options]
```

查看制品标签详情。

返回标签的详细信息，包括大小、摘要、创建时间等。Docker 制品支持通过 `--arch` 指定架构。

## 选项

- `<type>`: 制品类型（必填）
- `<name>`: 制品名称（必填）
- `<tag>`: 标签名（必填）
- `-r, --registry <PATH>`: 制品库路径（必填）
- `--arch <ARCH>`: 架构（Docker 制品时指定，如 `linux/amd64`）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# Docker 制品指定架构
$ cnb-rs registry tag detail docker my-app latest --registry my-org/my-registry --arch linux/amd64

# Helm Chart
$ cnb-rs registry tag detail helm my-chart 1.0.0 --registry my-org/helm-charts

# npm 包
$ cnb-rs registry tag detail npm my-lib 1.2.3 --registry my-org/npm-registry
```

## 另请参阅

- [cnb-rs registry](/registry/)
- [cnb-rs registry tag list](/registry/tag-list)
- [cnb-rs registry tag delete](/registry/tag-delete)
