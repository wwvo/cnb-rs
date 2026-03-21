---
title: cnb-rs auth switch
---

# cnb-rs auth switch

```
cnb-rs auth switch [<USERNAME>]
```

切换当前域名下的激活账号。

此命令不会重新请求 Token，也不会访问浏览器授权流程；它只会在本地已保存账号之间切换。
如果目标账号的 Token 位于系统凭证存储中，切换前会先验证该 Token 是否仍可读取。

::: tip
如果当前认证来自环境变量 `CNB_TOKEN` 或 `CNB_TOKEN_{DOMAIN}`，CLI 会拒绝切换本地账号。
请先清理环境变量，再执行此命令。
:::

## 选项

- `<USERNAME>`: 目标用户名；不提供时会列出当前域名下的已保存账号并提示选择

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 直接切换到指定账号
$ cnb-rs auth switch octocat
✓ 已切换到 octocat (cnb.cool)

# 交互式选择已保存账号
$ cnb-rs auth switch
请选择要切换的账号：
  1. alice [config] (当前)
  2. octocat [keyring]
请输入编号或用户名: 2
✓ 已切换到 octocat (cnb.cool)
```

## 错误处理

| 错误场景 | 错误信息 | 退出码 |
| --- | --- | --- |
| 没有已保存账号 | `当前域名没有已保存的登录账号：{domain}` | 1 |
| 账号不存在 | `未找到账号 {username}。可用账号：...` | 1 |
| keyring 中找不到账号 token | `无法切换到 {username}：系统凭证存储中未找到对应 token` | 1 |
| 环境变量认证生效中 | 提示先清理环境变量 | 1 |

## 另请参阅

- [cnb-rs auth](/auth/)
- [cnb-rs auth login](/auth/login)
- [cnb-rs auth status](/auth/status)
- [cnb-rs auth logout](/auth/logout)
