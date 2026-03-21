---
title: cnb-rs member group-update
---

# cnb-rs member group-update

```
cnb-rs member group-update <username> --group <group> --role <role>
```

更新组织成员权限。

## 选项

- `<username>`: 用户名（必填）
- `-g, --group <GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 新权限等级（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs member group-update zhangsan --group myorg --role Master
✓ 已更新成员 zhangsan 权限为 Master
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member group-list](/member/group-list)
- [cnb-rs member group-remove](/member/group-remove)
