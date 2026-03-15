# cnb-rs member collaborator-list

```
cnb-rs member collaborator-list --group <group> [options]
```

列出组织外部贡献者。

外部贡献者是被授予仓库级别权限但不属于组织成员的用户。

## 选项

- `-g, --group <GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 按角色过滤
- `-s, --search <KEYWORD>`: 搜索成员

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出组织外部贡献者
$ cnb-rs member collaborator-list --group myorg

# JSON 格式输出
$ cnb-rs member collaborator-list --group myorg --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member collaborator-update](/member/collaborator-update)
- [cnb-rs member collaborator-remove](/member/collaborator-remove)
