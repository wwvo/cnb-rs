# cnb group settings

```
cnb group settings <GROUP> [options]
```

查看或更新组织配置。不带修改选项时为查看模式，带修改选项时为更新模式。

## 参数

`GROUP`
: 组织路径（必填）

## 选项（更新模式）

`--hide-members <0|1>`
: 是否对外隐藏组织成员（0=否, 1=是）

`--hide-sub-groups <0|1>`
: 是否对外隐藏子组织（0=否, 1=是）

`--group-protection <0|1>`
: 组织保护开关（0=关闭, 1=开启）

`--show-watermark <0|1>`
: 是否显示私有仓库水印（0=否, 1=是）

## 输出（查看模式）

| 配置项 | 说明 |
|------|------|
| 隐藏成员 | 是否对外隐藏成员列表 |
| 隐藏子组织 | 是否对外隐藏子组织 |
| 组织保护 | 是否开启组织保护 |
| 私有仓库水印 | 是否显示私有仓库水印 |
| 可显示成员（上级） | 上级组织是否允许显示成员 |
| 可显示子组织（上级） | 上级组织是否允许显示子组织 |
| 邮箱验证 | 限制加入的邮箱域名 |

## 示例

```bash
# 查看组织配置
$ cnb group settings my-org

# 隐藏成员列表
$ cnb group settings my-org --hide-members 1

# 开启组织保护
$ cnb group settings my-org --group-protection 1

# JSON 输出
$ cnb group settings my-org --json
```

## API

| 场景 | 方法 | 端点 |
|------|------|------|
| 查看 | GET | `/{slug}/-/settings` |
| 更新 | PUT | `/{slug}/-/settings` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
