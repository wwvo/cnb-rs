# cnb group create

```
cnb group create <PATH> [options]
```

创建新组织。

## 参数

`PATH`
: 组织路径，作为唯一标识（必填）

## 选项

`-d, --description <TEXT>`
: 组织描述

`-r, --remark <TEXT>`
: 备注

`--bind-domain <DOMAIN>`
: 根组织绑定的域名

## 示例

```bash
# 创建基本组织
$ cnb group create my-org

# 带描述和备注
$ cnb group create my-org --description "我的组织" --remark "测试用"

# 绑定域名
$ cnb group create my-org --bind-domain example.com
```

## API

| 方法 | 端点 |
|------|------|
| POST | `/groups` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
