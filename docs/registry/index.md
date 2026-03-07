# cnb registry

```
cnb registry <subcommand>
```

管理组织下的制品库（Registry）和制品（Package），包括制品库查询、制品列表/详情/删除、标签管理、可见性设置等。

## 可用子命令

### 制品库管理

| 子命令          | 说明                 |
|-----------------|----------------------|
| list            | 列出组织下的制品库   |
| delete          | 删除制品库           |
| set-visibility  | 设置制品库可见性     |

### 制品管理

| 子命令           | 说明           |
|------------------|----------------|
| package list     | 列出制品       |
| package detail   | 查看制品详情   |
| package delete   | 删除制品       |

### 标签管理

| 子命令       | 说明             |
|--------------|------------------|
| tag list     | 列出制品标签     |
| tag detail   | 查看标签详情     |
| tag delete   | 删除制品标签     |

## 支持的制品类型

docker, docker_model, helm, npm, pypi, maven, nuget, cargo, composer, conan, generic, ohpm

## 示例

```bash
# 列出制品库
cnb registry list --group my-org

# 列出 Docker 制品
cnb registry package list docker --registry my-org/my-registry

# 查看标签详情
cnb registry tag detail docker my-app latest --registry my-org/my-registry --arch linux/amd64
```
