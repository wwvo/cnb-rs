# cnb repo contributor

```
cnb repo contributor [<repo>] [flags]
```

查看仓库贡献者排行及趋势数据，包括每位贡献者的提交数、新增行数和删除行数。

不指定仓库路径时，使用当前 Git 目录对应的仓库。

## 选项

`<repo>`
: 仓库路径（如 `org/repo`），可选

`--limit <N>`, `-L`
: 贡献者数量上限（默认：30，上限 100）

`--exclude-external`
: 排除外部用户

**继承的全局选项：**

`--json`
: 以 JSON 格式输出完整贡献者趋势数据

`--domain <DOMAIN>`
: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前仓库贡献者 Top 10
$ cnb repo contributor --limit 10

# 查看指定仓库
$ cnb repo contributor org/repo

# 排除外部用户
$ cnb repo contributor --exclude-external

# JSON 输出
$ cnb repo contributor --json
```

## API

| 步骤         | API                                                    | 方法 | 说明           |
| ------------ | ------------------------------------------------------ | ---- | -------------- |
| 获取趋势数据 | `${CNB_API_ENDPOINT}/{slug}/-/contributor/trend`       | GET  | 贡献者趋势     |

**API 详情**（OpenAPI：[`GetRepoContributorTrend`](https://api.cnb.cool/#/operations/GetRepoContributorTrend)）：

- **权限要求：** `repo-code:r`

## 另请参阅

- [cnb repo](/repo/)
- [cnb repo top-contributors](/repo/top-contributors)
- [cnb stats](/stats)
