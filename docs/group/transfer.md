# cnb group transfer

```
cnb group transfer <GROUP> --target <TARGET> [options]
```

将组织转移到另一个父组织下。**此操作需要交互确认**。

## 参数

`GROUP`
: 要转移的组织路径（必填）

## 选项

`-t, --target <TARGET>`
: 目标父组织路径（必填）

`--confirm`
: 跳过交互确认

## 示例

```bash
# 转移组织（需确认）
$ cnb group transfer child-org --target parent-org

# 跳过确认
$ cnb group transfer child-org --target parent-org --confirm
```

## API

| 方法 | 端点 |
|------|------|
| POST | `/{group}/-/transfer` |

## 另请参阅

- [cnb group](/group/)
