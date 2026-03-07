# cnb auth logout

```
cnb auth logout
```

退出登录，移除当前域名的认证配置。

仅移除本地存储的认证信息，**不会撤销远程 Token**。
如需撤销已生成的 Token，请访问 CNB 平台的 [个人设置 > 访问令牌](https://cnb.cool/profile/token) 页面。

根据 Token 来源执行不同操作：

- **配置文件** — 自动从 `~/.cnb/config.toml` 中移除对应域名的 `[auth.{domain}]` 段
- **域名特定环境变量**（`CNB_TOKEN_{DOMAIN}`）— 无法通过 CLI 移除，提示用户手动清除
- **通用环境变量**（`CNB_TOKEN`）— 无法通过 CLI 移除，提示用户手动清除
- **未登录** — 提示当前未登录

## 选项

无子命令特有选项。

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定要退出登录的目标域名（默认：`cnb.cool`）

## 示例

```bash
# 正常退出（Token 来自配置文件）
$ cnb auth logout
✓ 已退出 (cnb.cool)

# 指定域名退出
$ cnb --domain example.com auth logout
✓ 已退出 (example.com)

# Token 来自环境变量时（Unix）
$ CNB_TOKEN=xxx cnb auth logout
Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除
请手动执行：unset CNB_TOKEN

# Token 来自环境变量时（Windows PowerShell）
PS> $env:CNB_TOKEN="xxx"; cnb auth logout
Token 来自环境变量 CNB_TOKEN，无法通过 CLI 移除
请手动执行：$env:CNB_TOKEN=""

# Token 来自域名特定环境变量时
$ CNB_TOKEN_cnbcool=xxx cnb auth logout
Token 来自环境变量 CNB_TOKEN_cnbcool，无法通过 CLI 移除
请手动执行：unset CNB_TOKEN_cnbcool

# 未登录时
$ cnb auth logout
未登录 (cnb.cool)
```

## 另请参阅

- [cnb auth](/auth/)
- [cnb auth login](/auth/login)
- [cnb auth status](/auth/status)
