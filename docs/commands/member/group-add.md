# cnb-rs member group-add

```
cnb-rs member group-add <username> --group <group> --role <role>
```

添加组织成员。

将指定用户添加为组织成员并赋予相应权限等级。

## 选项

- `<username>`: 用户名（必填）
- `-g, --group <GROUP>`: 组织路径（必填）
- `-r, --role <ROLE>`: 权限等级，如 `Guest`、`Reporter`、`Developer`、`Master`（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs member group-add zhangsan --group myorg --role Developer
✓ 已添加成员 zhangsan
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member group-list](/member/group-list)
- [cnb-rs member group-update](/member/group-update)
