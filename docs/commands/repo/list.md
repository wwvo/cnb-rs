# cnb-rs repo list

```
cnb-rs repo list [<owner>] [flags]
```

列出用户或组织的仓库。

根据 `owner` 参数自动选择不同的 API：

- **无参数** — 列出当前认证用户的仓库
- **指定组织** — 列出组织下的仓库
- **指定用户** — 列出该用户的公开仓库

::: tip
当指定 `owner` 时，CLI 会先尝试作为组织查询；如果组织不存在（404），则作为用户查询。
:::

## 选项

- `[<owner>]`: 用户名或组织名（可选）
- `-L, --limit <N>`: 最大列出数量（默认：`30`，上限 `100`）
- `--visibility <TYPE>`: 按可见性过滤：`public`、`private`、`secret`
- `--sort <FIELD>`: 排序字段：`created_at`、`last_updated_at`、`stars`
- `--desc`: 倒序排列
- `-s, --search <KEYWORD>`: 按关键词搜索

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出当前用户的仓库
$ cnb-rs repo list

# 列出组织的仓库
$ cnb-rs repo list my-org

# 按更新时间倒序，限制 10 条
$ cnb-rs repo list --sort last_updated_at --desc --limit 10

# 搜索仓库
$ cnb-rs repo list --search "cli"

# 仅列出私有仓库
$ cnb-rs repo list --visibility private

# JSON 输出
$ cnb-rs repo list --json
```

## API

根据 `owner` 参数类型选择：

| 场景               | API                                          | 方法 | 说明               |
| ------------------ | -------------------------------------------- | ---- | ------------------ |
| 无参数（当前用户） | `${CNB_API_ENDPOINT}/user/repos`             | GET  | 列出当前用户的仓库 |
| 指定用户           | `${CNB_API_ENDPOINT}/users/{username}/repos` | GET  | 列出指定用户的仓库 |
| 指定组织           | `${CNB_API_ENDPOINT}/{slug}/-/repos`         | GET  | 列出组织下的仓库   |

**API 详情**（OpenAPI）：

- [`GetRepos`](https://api.cnb.cool/#/operations/GetRepos) — 当前用户仓库
- [`GetReposByUserName`](https://api.cnb.cool/#/operations/GetReposByUserName) — 指定用户仓库
- [`GetGroupSubRepos`](https://api.cnb.cool/#/operations/GetGroupSubRepos) — 组织仓库

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo view](/repo/view)
