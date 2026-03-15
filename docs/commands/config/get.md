# cnb-rs config get

```
cnb-rs config get <key>
```

获取指定配置项的当前值。

如果配置项未设置，输出其默认值并标注 `(default)`。

## 选项

- `<key>`: 配置项名称，可选值：`domain`、`git_protocol`

## 示例

```bash
# 获取域名配置
$ cnb-rs config get domain
cnb.cool

# 获取 Git 协议配置
$ cnb-rs config get git_protocol
https (default)
```

## 另请参阅

- [cnb-rs config](/config/)
- [cnb-rs config set](/config/set)
