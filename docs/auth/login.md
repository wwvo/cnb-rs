# cnb auth login

```
cnb auth login [flags]
```

登录 CNB 平台。

默认使用交互式隐藏输入方式获取 Token。也可以通过 `--token` 参数直接指定。

登录流程：

1. 获取 Token（参数传入或交互式隐藏输入）
2. 调用 `GET /user` API 验证 Token 有效性
3. 验证成功后将 Token 和用户名保存到 `~/.cnb/config.toml`

默认域名为 `cnb.cool`，可通过 `--domain` 全局参数覆盖。

CNB CLI 也支持通过环境变量传递 Token，适合 CI/CD 等无头环境。详见 [cnb auth](/auth/)。

::: tip
此命令不会撤销已有的 Token。如需管理已生成的 Token，请访问
CNB 平台的 [个人设置 > 访问令牌](https://cnb.cool/profile/token) 页面。
:::

## 选项

`--token <TOKEN>`
: 直接指定 Token，不提供则交互式隐藏输入

**继承的全局选项：**

`--domain <DOMAIN>`
: 指定目标域名（默认：`cnb.cool`）

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

## 示例

```bash
# 交互式登录（隐藏输入）
$ cnb auth login
请输入 Token（输入不会显示）: ********
正在验证 Token...
✓ 已登录为 octocat (cnb.cool)

# 直接指定 Token
$ cnb auth login --token cnb_xxxxxxxxxxxx
正在验证 Token...
✓ 已登录为 octocat (cnb.cool)

# 指定域名登录
$ cnb --domain example.com auth login
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
| 配置文件写入失败 | IO 错误信息               | 1      |

## 另请参阅

- [cnb auth](/auth/)
- [cnb auth status](/auth/status)
- [cnb auth logout](/auth/logout)
