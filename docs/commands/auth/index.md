---
title: cnb-rs auth
---

# cnb-rs auth

```
cnb-rs auth <subcommand>
```

认证管理，管理 CNB 平台的登录凭证。

cnb-rs 使用 Personal Access Token（PAT）进行认证。Token 通过 `cnb-rs auth login` 登录后
存储在本地配置文件 `~/.cnb/config.toml` 中。

也可以通过环境变量提供 Token，适合 CI/CD 等自动化场景。详见 [环境变量](#环境变量) 章节。

## 可用命令

- [cnb-rs auth login](/auth/login) — 登录 CNB 平台
- [cnb-rs auth status](/auth/status) — 查看当前认证状态
- [cnb-rs auth logout](/auth/logout) — 退出登录

## 环境变量

cnb-rs 按以下优先级查找认证 Token：

1. **域名特定环境变量** `CNB_TOKEN_{DOMAIN}`（域名去掉 `.` 和 `-`，如 `CNB_TOKEN_cnbcool`）
2. **通用环境变量** `CNB_TOKEN`
3. **配置文件** `~/.cnb/config.toml` 中 `[auth.{domain}]` 段的 `token` 字段

环境变量优先级高于配置文件，适合在 CI/CD 流水线中使用：

::: code-group

```bash [Unix / Linux / macOS]
export CNB_TOKEN=cnb_xxxxxxxxxxxx
cnb-rs issue list
```

```powershell [Windows PowerShell]
$env:CNB_TOKEN="cnb_xxxxxxxxxxxx"
cnb-rs issue list
```

```yaml [CNB 流水线]
env:
  CNB_TOKEN: ${{ secrets.CNB_TOKEN }}
```

:::

## 配置文件

Token 保存在 `~/.cnb/config.toml` 中，格式如下：

::: code-group

```toml [单域名]
domain = "cnb.cool"
git_protocol = "https"

[auth.cnb.cool]
token = "cnb_xxxxxxxxxxxx"
username = "octocat"
```

```toml [多域名]
domain = "cnb.cool"
git_protocol = "https"

[auth.cnb.cool]
token = "cnb_xxxxxxxxxxxx"
username = "octocat"

[auth.example.com]
token = "cnb_yyyyyyyyyyyy"
username = "alice"
```

:::

## 安全考虑

- Token **明文存储**在配置文件中（与 gh CLI 行为一致）
- 交互式输入使用 `rpassword` 隐藏回显
- `status` 命令对 Token 脱敏显示（仅保留前 4 位和后 4 位）
- 环境变量 Token 无法通过 CLI 移除，需用户手动 `unset`
- `logout` 命令仅移除本地配置，**不会撤销远程 Token**

## 另请参阅

- [cnb-rs](/cnb)
- [cnb-rs config](/config/)
