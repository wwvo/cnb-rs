# cnb group sub-groups

```
cnb group sub-groups <GROUP> [options]
```

列出指定组织下的子组织。与 `cnb group list --group` 不同，此命令调用的是子组织列表 API，不受权限过滤。

## 参数

`GROUP`
: 父组织路径（必填）

## 选项

`-s, --search <KEYWORD>`
: 关键字过滤

## 输出

表格输出包含以下列：

| 列 | 说明 |
|------|------|
| 路径 | 子组织路径 |
| 描述 | 子组织描述 |
| 仓库 | 直属仓库数量 |
| 成员 | 直属成员数量 |
| 子组织 | 直属子组织数量 |

## 示例

```bash
# 列出子组织
$ cnb group sub-groups my-org

# 搜索子组织
$ cnb group sub-groups my-org --search dev

# JSON 输出
$ cnb group sub-groups my-org --json
```

## API

| 方法 | 端点 |
|------|------|
| GET | `/{slug}/-/sub-groups` |

## 另请参阅

- [cnb group](/group/)
- [cnb group list](/group/list)
