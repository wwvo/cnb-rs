# cnb-rs member group-list

```
cnb-rs member group-list --group <group> [options]
```

列出组织直接成员。

输出为表格格式，包含用户名、昵称、权限等级等信息。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 按角色过滤
- `-s, --search <KEYWORD>`: 搜索成员

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出组织直接成员
$ cnb-rs member group-list --group myorg

# 按角色过滤
$ cnb-rs member group-list --group myorg --role Owner

# JSON 格式输出
$ cnb-rs member group-list --group myorg --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member group-add](/member/group-add)
- [cnb-rs member group-inherited](/member/group-inherited)
