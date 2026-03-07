# cnb gpg-key

```
cnb gpg-key <subcommand>
```

管理用户的 GPG 公钥，用于 Git 提交签名验证。

## 可用子命令

| 子命令 | 说明           |
|--------|----------------|
| list   | 列出 GPG 密钥  |

## 示例

```bash
# 列出所有 GPG 密钥
cnb gpg-key list

# 搜索密钥
cnb gpg-key list -k "My Key"

# JSON 输出
cnb gpg-key list --json
```

## GPG 签名验证

GPG 密钥用于验证 Git 提交签名：

1. 用户添加 GPG 公钥到 CNB 平台
2. 本地使用 `git commit -S` 签名提交
3. 推送到 CNB 后，平台验证签名并显示 "Verified" 标记

**前置条件：** GPG 密钥关联的邮箱必须与 CNB 账户邮箱匹配且已验证。

## 另请参阅

- [cnb auth](/auth/)
