# cnb group update

```
cnb group update <GROUP> [options]
```

更新组织信息。至少需要指定一个修改项。

## 参数

`GROUP`
: 组织路径（必填）

## 选项

`-d, --description <TEXT>`
: 更新描述

`-r, --remark <TEXT>`
: 更新备注

`-e, --email <EMAIL>`
: 更新联系邮箱

`-s, --site <URL>`
: 更新网站

`--domain <DOMAIN>`
: 更新域名

`--wechat-mp <ID>`
: 更新微信公众号

`--readme-repo <PATH>`
: 设置 README 仓库路径

## 示例

```bash
# 更新描述
$ cnb group update my-org --description "新的描述"

# 更新多个字段
$ cnb group update my-org --email contact@example.com --site https://example.com

# 设置 README 仓库
$ cnb group update my-org --readme-repo my-org/readme
```

## API

| 方法 | 端点 |
|------|------|
| PUT | `/{group}` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
