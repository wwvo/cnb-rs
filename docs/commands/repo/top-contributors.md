# cnb-rs repo top-contributors

```
cnb-rs repo top-contributors [<repo>] [flags]
```

查看仓库活跃用户排名（综合活动排名，区别于 `contributor` 的提交趋势统计）。

不指定仓库路径时，使用当前 Git 目录对应的仓库。

## 选项

- `[<repo>]`: 仓库路径（如 `org/repo`），可选
- `-n, --top <N>`: 返回用户数量（默认：`10`）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前仓库 Top 10 活跃用户
$ cnb-rs repo top-contributors

# 指定数量
$ cnb-rs repo top-contributors --top 5

# 查看指定仓库
$ cnb-rs repo top-contributors org/repo

# JSON 输出
$ cnb-rs repo top-contributors --json
```

## API

| 步骤         | API                                               | 方法 | 说明         |
| ------------ | ------------------------------------------------- | ---- | ------------ |
| 获取活跃用户 | `${CNB_API_ENDPOINT}/{repo}/-/top-activity-users` | GET  | 活跃用户排名 |

**API 详情**（OpenAPI：[`TopContributors`](https://api.cnb.cool/#/operations/TopContributors)）：

- **权限要求：** `repo-base-info:r`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo contributor](/repo/contributor)
