# cnb-rs user

```
cnb-rs user <subcommand>
```

查看用户的社交关系和活动数据。

支持查看粉丝/关注列表，以及用户在 CNB 平台上的活动汇总和详情。
所有子命令均支持查看其他用户的数据（通过 `<username>` 参数），不指定则默认查看当前认证用户。

## 可用命令

- [cnb-rs user followers](/user/followers) — 查看粉丝列表
- [cnb-rs user following](/user/following) — 查看关注列表
- [cnb-rs user activities](/user/activities) — 查看活动汇总
- [cnb-rs user activity-detail](/user/activity-detail) — 查看仓库活动详情

## 示例

```bash
# 查看当前用户的粉丝
$ cnb-rs user followers

# 查看指定用户的关注列表
$ cnb-rs user following zhangsan

# 查看当月活动汇总
$ cnb-rs user activities

# 查看仓库活动详情
$ cnb-rs user activity-detail --type commit --repo org/repo --date 202501
```

## 另请参阅

- [cnb-rs](/guide/cnb)
- [cnb-rs auth](/auth/)
