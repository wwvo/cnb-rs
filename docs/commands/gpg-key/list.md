---
title: cnb-rs gpg-key list
---

# cnb-rs gpg-key list

```
cnb-rs gpg-key list [--keyword <text>]
```

列出当前用户的 GPG 密钥。

## 选项

- `-k, --keyword <TEXT>`: 搜索关键字

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs gpg-key list
KEY ID              NAME              EMAILS                     EXPIRES     SUBKEYS
ABCD1234EF567890    My GPG Key        user@example.com (✓)       2026-01-01  2
1234567890ABCDEF    Work Key          work@company.com (✓)       never       1

# 搜索密钥
$ cnb-rs gpg-key list -k "Work"

# JSON 格式输出
$ cnb-rs gpg-key list --json
```

## 另请参阅

- [cnb-rs gpg-key](/gpg-key/)
- [cnb-rs auth](/auth/)
