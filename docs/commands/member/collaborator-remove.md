---
title: cnb-rs member collaborator-remove
---

# cnb-rs member collaborator-remove

```
cnb-rs member collaborator-remove <username> --group <group> [options]
```

移除外部贡献者。

执行前会要求确认，可通过 `--yes` 跳过确认提示。

## 选项

- `<username>`: 用户名（必填）
- `-g, --group <GROUP>`: 组织路径（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 移除外部贡献者（需确认）
$ cnb-rs member collaborator-remove zhangsan --group myorg

# 跳过确认
$ cnb-rs member collaborator-remove zhangsan --group myorg --yes
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member collaborator-list](/member/collaborator-list)
