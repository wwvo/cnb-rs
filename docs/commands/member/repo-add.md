# cnb-rs member repo-add

```
cnb-rs member repo-add <username> --role <role> [options]
```

添加仓库成员。

将指定用户添加为仓库成员并赋予相应权限等级。可通过 `--outside-collaborator` 标记为外部贡献者。

## 选项

- `<username>`: 用户名（必填）
- `-r, --role <ROLE>`: 权限等级，如 `Guest`、`Reporter`、`Developer`、`Master`（必填）
- `--outside-collaborator`: 标记为外部贡献者

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 添加仓库成员
$ cnb-rs member repo-add zhangsan --role Developer
✓ 已添加成员 zhangsan

# 添加为外部贡献者
$ cnb-rs member repo-add lisi --role Reporter --outside-collaborator
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-list](/member/repo-list)
- [cnb-rs member repo-update](/member/repo-update)
