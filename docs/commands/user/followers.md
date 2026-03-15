# cnb-rs user followers

```
cnb-rs user followers [<username>] [options]
```

查看指定用户的粉丝列表，不指定则查看当前认证用户。

输出为表格格式，包含用户名和昵称。

## 选项

- `[<username>]`: 用户名（不指定则查看当前用户）
- `-L, --limit <N>`: 最大返回数量（默认：`100`）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前用户的粉丝
$ cnb-rs user followers

# 查看指定用户的粉丝
$ cnb-rs user followers zhangsan

# 限制返回数量
$ cnb-rs user followers --limit 20

# JSON 格式输出
$ cnb-rs user followers --json
```

## 另请参阅

- [cnb-rs user](/user/)
- [cnb-rs user following](/user/following)
