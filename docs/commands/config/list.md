# cnb-rs config list

```
cnb-rs config list
```

列出所有配置项及当前值。

遍历所有支持的配置项（`domain`、`git_protocol`），显示已设置的值。未设置的项显示默认值并标注 `(default)`。

默认值：

- `domain` — `cnb.cool`
- `git_protocol` — `https`

## 输出示例

全部使用默认值：

```
domain = cnb.cool (default)
git_protocol = https (default)
```

自定义后：

```
domain = example.com
git_protocol = ssh
```

## 另请参阅

- [cnb-rs config](/config/)
