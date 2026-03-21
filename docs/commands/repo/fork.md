---
title: cnb-rs repo fork
---

# cnb-rs repo fork

```
cnb-rs repo fork [<repo>] [flags]
```

查看仓库的 Fork 列表。

不指定仓库路径时，使用当前 Git 目录对应的仓库。

::: info
目前 OpenAPI 仅提供 Fork **列表查询**接口，暂不支持通过 CLI 创建 Fork。
:::

## 选项

- `[<repo>]`: 仓库路径（如 `org/repo`），可选
- `-L, --limit <N>`: 最大列出数量（默认：`30`，上限 `100`）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看当前仓库的 Fork 列表
$ cnb-rs repo fork

# 查看指定仓库的 Fork
$ cnb-rs repo fork org/repo

# 限制数量
$ cnb-rs repo fork org/repo --limit 10

# JSON 输出
$ cnb-rs repo fork --json
```

## API

| 步骤      | API                                  | 方法 | 说明           |
| --------- | ------------------------------------ | ---- | -------------- |
| 获取 Fork | `${CNB_API_ENDPOINT}/{repo}/-/forks` | GET  | 获取 Fork 列表 |

**API 详情**（OpenAPI：[`ListForksRepos`](https://api.cnb.cool/#/operations/ListForksRepos)）：

- **权限要求：** `repo-base-info:r`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo view](/repo/view)
