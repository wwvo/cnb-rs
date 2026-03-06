# cnb group delete

```
cnb group delete <GROUP> [options]
```

删除指定组织。**此操作不可逆**，需要交互确认。

## 参数

`GROUP`
: 组织路径（必填）

## 选项

`--confirm`
: 跳过交互确认，直接删除

## 示例

```bash
# 删除组织（需确认）
$ cnb group delete my-org

# 跳过确认直接删除
$ cnb group delete my-org --confirm
```

## 错误处理

| 错误 | 说明 |
|------|------|
| 403 | 无权限删除该组织 |
| 404 | 组织不存在 |

## API

| 方法 | 端点 |
|------|------|
| DELETE | `/{group}` |

## 另请参阅

- [cnb group](/group/)
- [cnb group create](/group/create)
