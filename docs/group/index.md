# cnb group

组织管理。支持组织的创建、查看、更新、删除、转移，以及子组织管理、配置管理、额度查看、成员管理等。

## 可用命令

| 命令                                        | 说明              |
| ------------------------------------------- | ----------------- |
| [cnb group list](/group/list)               | 列出我的组织      |
| [cnb group view](/group/view)               | 查看组织详情      |
| [cnb group create](/group/create)           | 创建组织          |
| [cnb group update](/group/update)           | 更新组织信息      |
| [cnb group delete](/group/delete)           | 删除组织          |
| [cnb group transfer](/group/transfer)       | 转移组织          |
| [cnb group sub-groups](/group/sub-groups)   | 列出子组织        |
| [cnb group settings](/group/settings)       | 查看/更新组织配置 |
| [cnb group quota](/group/quota)             | 查看组织特权额度  |
| [cnb group member](/group/member)           | 组织成员管理      |
| [cnb group update-logo](/group/update-logo) | 更新组织 Logo     |

## 全局选项

`--json`
: 以 JSON 格式输出（适用于 list、view、sub-groups、settings、quota、member list）

`--domain <DOMAIN>`
: 指定 CNB 平台域名

## 示例

```bash
# 列出我的顶层组织
$ cnb group list

# 查看组织详情
$ cnb group view my-org

# 创建新组织
$ cnb group create my-new-org --description "我的新组织"

# 管理成员
$ cnb group member list my-org
$ cnb group member add my-org alice --level Developer
```

## 另请参阅

- [cnb](/guide/cnb)
