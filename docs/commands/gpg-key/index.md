---
title: cnb-rs gpg-key
---

# cnb-rs gpg-key

```
cnb-rs gpg-key <subcommand>
```

管理用户的 GPG 公钥，用于 Git 提交签名验证。

GPG 密钥用于验证 Git 提交签名：用户添加 GPG 公钥到 CNB 平台后，本地使用 `git commit -S` 签名提交，推送到 CNB 后平台验证签名并显示 "Verified" 标记。GPG 密钥关联的邮箱必须与 CNB 账户邮箱匹配且已验证。

## 可用命令

- [cnb-rs gpg-key list](/gpg-key/list) — 列出 GPG 密钥

## 示例

```bash
# 列出所有 GPG 密钥
$ cnb-rs gpg-key list

# 搜索密钥
$ cnb-rs gpg-key list -k "My Key"

# JSON 格式输出
$ cnb-rs gpg-key list --json
```

## 另请参阅

- [cnb-rs](/cnb)
- [cnb-rs auth](/auth/)
