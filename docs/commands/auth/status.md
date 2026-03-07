# cnb auth status

```
cnb auth status
```

显示当前认证状态。

按优先级查找 Token 及其来源，然后调用 `GET /user` API 验证 Token 有效性并获取用户名，
最终显示域名、用户名、Token（脱敏）及来源信息。

如果未找到 Token，输出未登录提示并建议使用 `cnb auth login` 或设置环境变量。

::: tip
Token 脱敏规则：长度大于 12 时保留前 4 位和后 4 位，中间用 `****` 替代；
否则整体显示为 `****`。
:::

## 选项

无子命令特有选项。

**继承的全局选项：**

- `--domain <DOMAIN>`: 检查指定域名的认证状态（默认：`cnb.cool`）

## Token 来源优先级

CNB CLI 按以下顺序查找 Token：

1. **域名特定环境变量** — `CNB_TOKEN_{DOMAIN}`（域名去掉 `.` 和 `-`，如 `CNB_TOKEN_cnbcool`）
2. **通用环境变量** — `CNB_TOKEN`
3. **配置文件** — `~/.cnb/config.toml` 中 `[auth.{domain}]` 段的 `token` 字段

详见 [cnb auth — 环境变量](/auth/#环境变量)。

## 示例

```bash
# 已登录状态（Token 来自配置文件）
$ cnb auth status
域名：  cnb.cool
用户：  octocat
Token:  cnb_****xxxx（来源：配置文件 ~/.cnb/config.toml）

# 使用环境变量认证
$ CNB_TOKEN=cnb_xxxxxxxxxxxx cnb auth status
域名：  cnb.cool
用户：  octocat
Token:  cnb_****xxxx（来源：环境变量 CNB_TOKEN）

# 使用域名特定环境变量
$ CNB_TOKEN_cnbcool=cnb_xxxxxxxxxxxx cnb auth status
域名：  cnb.cool
用户：  octocat
Token:  cnb_****xxxx（来源：环境变量 CNB_TOKEN_cnbcool）

# Token 无效
$ cnb auth status
域名：  cnb.cool
用户：  (Token 无效)
Token:  cnb_****xxxx（来源：配置文件 ~/.cnb/config.toml）

# 未登录
$ cnb auth status
未登录 (cnb.cool)
使用 `cnb auth login` 登录，或设置环境变量 CNB_TOKEN

# 检查指定域名
$ cnb --domain example.com auth status
未登录 (example.com)
使用 `cnb auth login` 登录，或设置环境变量 CNB_TOKEN
```

## API

| 步骤       | API                        | 方法 | 说明                    |
| ---------- | -------------------------- | ---- | ----------------------- |
| 获取用户名 | `${CNB_API_ENDPOINT}/user` | GET  | 验证 Token 并获取用户名 |

API 详情同 [cnb auth login — API](/auth/login#api) 章节。

## 另请参阅

- [cnb auth](/auth/)
- [cnb auth login](/auth/login)
- [cnb auth logout](/auth/logout)
