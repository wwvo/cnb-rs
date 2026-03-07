# cnb user

```
cnb user <subcommand>
```

查看用户的社交关系和活动数据。

## 可用子命令

| 子命令          | 说明             |
|-----------------|------------------|
| followers       | 查看粉丝列表     |
| following       | 查看关注列表     |
| activities      | 查看活动汇总     |
| activity-detail | 查看仓库活动详情 |

## 示例

```bash
# 查看当前用户的粉丝
cnb user followers

# 查看指定用户的关注列表
cnb user following zhangsan

# 查看当月活动汇总
cnb user activities

# 查看仓库活动详情
cnb user activity-detail --type commit --repo org/repo --date 202501
```
