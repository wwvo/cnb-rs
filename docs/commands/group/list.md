# cnb-rs group list

```
cnb-rs group list [options]
```

列出当前用户有权限的组织。

默认列出顶层组织，可通过 `--group` 指定父组织查看其子组织。

## 选项

- `-g, --group <GROUP>`: 指定父组织路径，列出该组织下的子组织
- `-s, --search <KEYWORD>`: 关键字过滤
- `-r, --role <ROLE>`: 按角色过滤：`Guest`、`Reporter`、`Developer`、`Master`、`Owner`

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出顶层组织
$ cnb-rs group list

# 按关键字搜索
$ cnb-rs group list --search my-org

# 列出指定组织下的子组织
$ cnb-rs group list --group my-org

# 按角色过滤
$ cnb-rs group list --role Owner

# JSON 格式输出
$ cnb-rs group list --json
```

## 另请参阅

- [cnb-rs group](/group/)
- [cnb-rs group view](/group/view)
