---
title: cnb-rs auth
---

# cnb-rs auth

```
cnb-rs auth <subcommand>
```

认证管理，管理 CNB 平台的登录凭证。

cnb-rs 使用 Personal Access Token（PAT）进行认证。执行 `cnb-rs auth login` 时，
默认会将 Token 保存到系统凭证存储；仅在凭证存储不可用，或显式使用
`--insecure-storage` 时，才会回退为写入本地配置文件 `~/.cnb/config.toml`。

也可以通过环境变量提供 Token，适合 CI/CD 等自动化场景。详见 [环境变量](#环境变量) 章节。

## 可用命令

- [cnb-rs auth login](/auth/login) — 登录 CNB 平台
- [cnb-rs auth switch](/auth/switch) — 切换当前域名下的已保存账号
- [cnb-rs auth status](/auth/status) — 查看当前认证状态
- [cnb-rs auth logout](/auth/logout) — 退出登录

## 环境变量

cnb-rs 按以下优先级查找认证 Token：

1. **域名特定环境变量** `CNB_TOKEN_{DOMAIN}`（域名去掉 `.` 和 `-`，如 `CNB_TOKEN_cnbcool`）
2. **通用环境变量** `CNB_TOKEN`
3. **系统凭证存储** 当前激活账号对应的 token
4. **配置文件** `~/.cnb/config.toml` 中当前激活账号的明文 token

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

配置文件中保存的是账号元数据和当前激活账号。只有明文回退模式下，Token 才会出现在配置文件里。
格式如下：

::: code-group

```toml [默认 keyring]
domain = "cnb.cool"
git_protocol = "https"

[auth."cnb.cool"]
user = "octocat"

[auth."cnb.cool".users.octocat]
storage = "keyring"
```

```toml [多账号与明文回退]
domain = "cnb.cool"
git_protocol = "https"

[auth."cnb.cool"]
user = "alice"

[auth."cnb.cool".users.alice]
storage = "config"
token = "cnb_xxxxxxxxxxxx"

[auth."cnb.cool".users.octocat]
storage = "keyring"

[auth."example.com"]
user = "bob"

[auth."example.com".users.bob]
storage = "keyring"
```

:::

## 安全考虑

- 默认使用系统凭证存储，避免将 Token 直接写入磁盘
- 当系统凭证存储不可用，或显式使用 `--insecure-storage` 时，Token 才会明文写入配置文件
- 交互式输入使用 `rpassword` 隐藏回显
- 支持为同一域名保存多个账号，并通过 `auth switch` 切换激活账号
- `status` 命令对 Token 脱敏显示（仅保留前 4 位和后 4 位）
- 环境变量 Token 无法通过 CLI 移除，需用户手动 `unset`
- `logout` 命令仅移除本地配置，**不会撤销远程 Token**

## 另请参阅

- [cnb-rs](/cnb)
- [cnb-rs config](/config/)
