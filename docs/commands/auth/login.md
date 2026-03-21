---
title: cnb-rs auth login
---

# cnb-rs auth login

```
cnb-rs auth login [flags]
```

登录 CNB 平台。

默认使用交互式隐藏输入方式获取 Token。也可以通过 `--with-token` 从标准输入读取，
或通过 `--token` 直接指定。

登录流程：

1. 获取 Token（参数传入或交互式隐藏输入）
2. 调用 `GET /user` API 验证 Token 有效性
3. 默认尝试将 Token 保存到系统凭证存储
4. 如果系统凭证存储不可用，自动回退为写入 `~/.cnb/config.toml`

默认域名为 `cnb.cool`，可通过 `--domain` 全局参数覆盖。

cnb-rs 也支持通过环境变量传递 Token，适合 CI/CD 等无头环境。详见 [cnb-rs auth](/auth/)。

::: tip
如果当前正在使用 `CNB_TOKEN` 或 `CNB_TOKEN_{DOMAIN}` 认证，此命令会拒绝写入本地凭证，
并提示先清理环境变量。
:::

::: tip
此命令不会撤销已有的远程 Token。如需管理已生成的 Token，请访问
CNB 平台的 [个人设置 > 访问令牌](https://cnb.cool/profile/token) 页面。
:::

## 选项

- `--with-token`: 从标准输入读取 Token
- `--token <TOKEN>`: 直接指定 Token；适合本地调试，不推荐用于长期脚本
- `--insecure-storage`: 将认证信息明文保存到 `~/.cnb/config.toml`

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 交互式登录（隐藏输入）
$ cnb-rs auth login
请输入 Token（输入不会显示）: ********
正在验证 Token...
✓ 已登录为 octocat (cnb.cool)

# 从标准输入读取 Token
$ cat mytoken.txt | cnb-rs auth login --with-token
正在验证 Token...
✓ 已登录为 octocat (cnb.cool)

# 直接指定 Token
$ cnb-rs auth login --token cnb_xxxxxxxxxxxx
正在验证 Token...
✓ 已登录为 octocat (cnb.cool)

# 强制明文存储
$ cnb-rs auth login --token cnb_xxxxxxxxxxxx --insecure-storage
正在验证 Token...
⚠ 认证信息已明文保存到 ~/.cnb/config.toml
✓ 已登录为 octocat (cnb.cool)

# 指定域名登录
$ cnb-rs --domain example.com auth login
请输入 Token（输入不会显示）: ********
正在验证 Token...
✓ 已登录为 alice (example.com)
```

## 错误处理

| 错误场景         | 错误信息                  | 退出码 |
| ---------------- | ------------------------- | ------ |
| Token 为空       | `Token 不能为空`          | 1      |
| Token 无效       | `Token 验证失败：{error}` | 1      |
| 网络不可达       | `Token 验证失败：{error}` | 1      |
| 环境变量认证生效中 | 提示先清理环境变量 | 1 |
| 配置文件写入失败 | IO 错误信息               | 1      |

## API

登录过程调用以下 API 验证 Token 有效性：

| 步骤       | API                        | 方法 | 说明                       |
| ---------- | -------------------------- | ---- | -------------------------- |
| Token 验证 | `${CNB_API_ENDPOINT}/user` | GET  | 获取当前用户信息验证有效性 |

**API 详情**（OpenAPI：[`GetUserInfo`](https://api.cnb.cool/#/operations/GetUserInfo)）：

- **权限要求：** `account-profile:r`
- **请求头：**
  - `Accept: application/vnd.cnb.api+json`
  - `Authorization: Bearer $CNB_TOKEN`
- **响应类型：** `dto.UsersResultForSelf`
- **关键响应字段：**

```json
{
  "id": "string",
  "username": "string",
  "nickname": "string",
  "email": "string",
  "avatar": "string",
  "bio": "string",
  "company": "string",
  "created_at": "string",
  "repo_count": 0,
  "group_count": 0,
  "stars_count": 0,
  "verified": 0
}
```

**cURL 示例：**

```bash
curl -X GET \
  "https://api.cnb.cool/user" \
  -H "Accept: application/vnd.cnb.api+json" \
  -H "Authorization: Bearer $CNB_TOKEN"
```

## 另请参阅

- [cnb-rs auth](/auth/)
- [cnb-rs auth switch](/auth/switch)
- [cnb-rs auth status](/auth/status)
- [cnb-rs auth logout](/auth/logout)
