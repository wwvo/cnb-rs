# cnb gpg-key list

```
cnb gpg-key list [--keyword <text>]
```

列出当前用户的 GPG 密钥。

## 参数

| 参数               | 缩写 | 说明       |
|--------------------|------|------------|
| `--keyword <text>` | `-k` | 搜索关键字 |

## 输出列

| 列名    | 说明                       |
|---------|----------------------------|
| KEY ID  | 公钥 KeyID                 |
| NAME    | 密钥标题                   |
| EMAILS  | 关联邮箱及验证状态（✓/✗）  |
| EXPIRES | 过期时间（或 never）       |
| SUBKEYS | 子密钥数量                 |

## 示例

```bash
cnb gpg-key list
# KEY ID              NAME              EMAILS                     EXPIRES     SUBKEYS
# ABCD1234EF567890    My GPG Key        user@example.com (✓)       2026-01-01  2
# 1234567890ABCDEF    Work Key          work@company.com (✓)       never       1

cnb gpg-key list -k "Work"

cnb gpg-key list --json
```

## 另请参阅

- [cnb gpg-key](/gpg-key/)
