---
title: cnb-rs member group-user-access
---

# cnb-rs member group-user-access

```
cnb-rs member group-user-access <username> --group <group>
```

查看指定成员在组织的权限层级。

显示该用户在组织中的直接权限和通过上级组织继承获得的权限层级。

## 选项

- `<username>`: 用户名（必填）
- `-g, --group <GROUP>`: 组织路径（必填）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs member group-user-access zhangsan --group myorg

# JSON 格式输出
$ cnb-rs member group-user-access zhangsan --group myorg --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member group-access-level](/member/group-access-level)
