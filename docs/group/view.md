# cnb group view

```
cnb group view <GROUP> [options]
```

查看指定组织的详细信息。

## 参数

`GROUP`
: 组织路径（必填）

## 选项

`-w, --web`
: 在浏览器中打开组织页面

## 输出

表格输出包含以下字段：

| 字段 | 说明 |
|------|------|
| 名称 | 组织展示名称 |
| 路径 | 组织路径标识 |
| 描述 | 组织描述 |
| 备注 | 组织备注 |
| 域名 | 绑定域名 |
| 邮箱 | 联系邮箱 |
| 网站 | 组织网站 |
| 成员 | 直属成员数 (总计) |
| 子组织 | 直属子组织数 (总计) |
| 仓库 | 直属仓库数 (总计) |
| 关注者 | 关注者数量 |
| 创建时间 | 组织创建时间 |

## 示例

```bash
# 查看组织详情
$ cnb group view my-org

# 在浏览器中打开
$ cnb group view my-org --web

# JSON 输出
$ cnb group view my-org --json
```

## API

| 方法 | 端点 |
|------|------|
| GET | `/{group}` |

## 另请参阅

- [cnb group](/group/)
- [cnb group list](/group/list)
