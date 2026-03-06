# cnb group list

```
cnb group list [options]
```

列出当前用户有权限的组织。默认列出顶层组织，可通过 `--group` 指定父组织查看其子组织。

## 选项

`-g, --group <GROUP>`
: 指定父组织路径，列出该组织下的子组织

`-s, --search <KEYWORD>`
: 关键字过滤

`-r, --role <ROLE>`
: 按角色过滤（Guest/Reporter/Developer/Master/Owner）

## 输出

表格输出包含以下列：

| 列 | 说明 |
|------|------|
| 路径 | 组织路径 |
| 描述 | 组织描述 |
| 仓库 | 直属仓库数量 |
| 成员 | 直属成员数量 |
| 子组织 | 直属子组织数量 |

## 示例

```bash
# 列出顶层组织
$ cnb group list

# 按关键字搜索
$ cnb group list --search my-org

# 列出指定组织下的子组织
$ cnb group list --group my-org

# 按角色过滤
$ cnb group list --role Owner

# JSON 输出
$ cnb group list --json
```

## API

| 场景 | 方法 | 端点 |
|------|------|------|
| 顶层组织 | GET | `/user/groups` |
| 子组织 | GET | `/user/groups/{slug}` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
