---
title: cnb-rs member collaborator-update
---

# cnb-rs member collaborator-update

```
cnb-rs member collaborator-update <username> --group <group> --role <role>
```

更新外部贡献者权限。

## 选项

- `<username>`: 用户名（必填）
- `-g, --group <GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 新权限等级（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs member collaborator-update zhangsan --group myorg --role Developer
✓ 已更新外部贡献者 zhangsan 权限为 Developer
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member collaborator-list](/member/collaborator-list)
- [cnb-rs member collaborator-remove](/member/collaborator-remove)
