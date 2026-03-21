---
title: cnb-rs auth logout
---

# cnb-rs auth logout

```
cnb-rs auth logout
```

退出登录，移除当前域名下的激活账号。

仅移除本地存储的认证信息，**不会撤销远程 Token**。
如需撤销已生成的 Token，请访问 CNB 平台的 [个人设置 > 访问令牌](https://cnb.cool/profile/token) 页面。

根据 Token 来源执行不同操作：

- **系统凭证存储** — 从系统 keyring 中删除当前账号的 token，并更新本地账号元数据
- **配置文件** — 从 `~/.cnb/config.toml` 中删除当前账号的明文 token 与账号元数据
- **域名特定环境变量**（`CNB_TOKEN_{DOMAIN}`）— 无法通过 CLI 移除，提示用户手动清除
- **通用环境变量**（`CNB_TOKEN`）— 无法通过 CLI 移除，提示用户手动清除
- **未登录** — 提示当前未登录

如果当前域名下还保存了其他账号，移除当前账号后会自动切换到剩余账号中的第一个。

## 选项

无子命令特有选项。

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定要退出登录的目标域名（默认：`cnb.cool`）

## 示例

```bash
# 正常退出（当前账号来自系统凭证存储或配置文件）
$ cnb-rs auth logout
✓ 已退出 octocat (cnb.cool)

# 指定域名退出
$ cnb-rs --domain example.com auth logout
✓ 已退出 alice (example.com)

# 退出当前账号后自动切换到其他已保存账号
$ cnb-rs auth logout
✓ 已退出 alice (cnb.cool)，当前账号已切换为 octocat

# Token 来自环境变量时（Unix）
$ CNB_TOKEN=xxx cnb-rs auth logout
Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除
请手动执行：unset CNB_TOKEN

# Token 来自环境变量时（Windows PowerShell）
PS> $env:CNB_TOKEN="xxx"; cnb-rs auth logout
Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除
请手动执行：$env:CNB_TOKEN=""

# Token 来自域名特定环境变量时
$ CNB_TOKEN_cnbcool=xxx cnb-rs auth logout
Token 来自环境变量 CNB_TOKEN_cnbcool，无法通过 CLI 移除
请手动执行：unset CNB_TOKEN_cnbcool

# 未登录时
$ cnb-rs auth logout
未登录 (cnb.cool)
```

## 另请参阅

- [cnb-rs auth](/auth/)
- [cnb-rs auth login](/auth/login)
- [cnb-rs auth switch](/auth/switch)
- [cnb-rs auth status](/auth/status)
