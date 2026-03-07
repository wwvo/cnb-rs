# cnb group delete

```
cnb group delete <GROUP> [options]
```

删除指定组织。**此操作不可逆**，需要交互确认。

## 选项

- `<GROUP>`: 组织路径（必填）
- `--confirm`: 跳过交互确认，直接删除

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除组织（需确认）
$ cnb group delete my-org

# 跳过确认直接删除
$ cnb group delete my-org --confirm
```

## 另请参阅

- [cnb group](/group/)
- [cnb group create](/group/create)
