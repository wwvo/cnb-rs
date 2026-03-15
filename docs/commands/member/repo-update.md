# cnb-rs member repo-update

```
cnb-rs member repo-update <username> --role <role>
```

更新仓库成员权限。

## 选项

- `<username>`: 用户名（必填）
- `-r, --role <ROLE>`: 新权限等级（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs member repo-update zhangsan --role Master
✓ 已更新成员 zhangsan 权限为 Master
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-list](/member/repo-list)
- [cnb-rs member repo-remove](/member/repo-remove)
