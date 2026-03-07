# cnb config set

```
cnb config set <key> <value>
```

设置配置项的值并写入 `~/.cnb/config.toml`。

如果配置文件或其父目录不存在，会自动创建。传入不在可用配置项列表中的 key 会报错。

## 选项

- `<key>`: 配置项名称，可选值：`domain`、`git_protocol`
- `<value>`: 配置项的值

## 示例

```bash
# 设置域名
$ cnb config set domain cnb.cool
✓ domain = cnb.cool

# 设置 Git 协议为 SSH
$ cnb config set git_protocol ssh
✓ git_protocol = ssh

# 传入无效 key
$ cnb config set foo bar
Error: 未知配置项：foo
可用配置项：domain, git_protocol
```

## 另请参阅

- [cnb config](/config/)
