# cnb-rs registry

```
cnb-rs registry <subcommand>
```

管理组织下的制品库（Registry）和制品（Package）。

制品库是 CNB 平台中存储和分发容器镜像、Helm Chart、npm 包等制品的服务。
支持 Docker、Helm、npm、PyPI、Maven、NuGet、Cargo 等多种制品类型。

## 可用命令

### 制品库管理

- [cnb-rs registry list](/registry/list) — 列出组织下的制品库
- [cnb-rs registry delete](/registry/delete) — 删除制品库
- [cnb-rs registry set-visibility](/registry/set-visibility) — 设置制品库可见性

### 制品管理

- [cnb-rs registry package list](/registry/package-list) — 列出制品
- [cnb-rs registry package detail](/registry/package-detail) — 查看制品详情
- [cnb-rs registry package delete](/registry/package-delete) — 删除制品

### 标签管理

- [cnb-rs registry tag list](/registry/tag-list) — 列出制品标签
- [cnb-rs registry tag detail](/registry/tag-detail) — 查看标签详情
- [cnb-rs registry tag delete](/registry/tag-delete) — 删除制品标签

## 支持的制品类型

`docker`、`docker_model`、`helm`、`npm`、`pypi`、`maven`、`nuget`、`cargo`、`composer`、`conan`、`generic`、`ohpm`

## 示例

```bash
# 列出制品库
$ cnb-rs registry list --group my-org

# 列出 Docker 制品
$ cnb-rs registry package list docker --registry my-org/my-registry

# 查看标签详情
$ cnb-rs registry tag detail docker my-app latest --registry my-org/my-registry --arch linux/amd64
```

## 另请参阅

- [cnb-rs](/guide/cnb)
- [cnb-rs build](/build/)
