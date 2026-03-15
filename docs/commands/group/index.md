# cnb-rs group

```
cnb-rs group <subcommand>
```

管理组织。

支持组织的创建、查看、更新、删除、转移，以及子组织管理、配置管理、额度查看、成员管理等。

## 可用命令

- [cnb-rs group list](/group/list) — 列出我的组织
- [cnb-rs group view](/group/view) — 查看组织详情
- [cnb-rs group create](/group/create) — 创建组织
- [cnb-rs group update](/group/update) — 更新组织信息
- [cnb-rs group delete](/group/delete) — 删除组织
- [cnb-rs group transfer](/group/transfer) — 转移组织
- [cnb-rs group sub-groups](/group/sub-groups) — 列出子组织
- [cnb-rs group settings](/group/settings) — 查看/更新组织配置
- [cnb-rs group quota](/group/quota) — 查看组织特权额度
- [cnb-rs group member](/group/member) — 组织成员管理
- [cnb-rs group update-logo](/group/update-logo) — 更新组织 Logo

## 示例

```bash
# 列出我的顶层组织
$ cnb-rs group list

# 查看组织详情
$ cnb-rs group view my-org

# 创建新组织
$ cnb-rs group create my-new-org --description "我的新组织"

# 管理成员
$ cnb-rs group member list my-org
$ cnb-rs group member add my-org alice --level Developer
```

## 另请参阅

- [cnb-rs](/guide/cnb)
- [cnb-rs member](/member/)
