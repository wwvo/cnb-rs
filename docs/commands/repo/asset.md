---
title: cnb-rs repo asset
---

# cnb-rs repo asset

```
cnb-rs repo asset <list|delete> [flags]
```

管理仓库资产记录，包括 release 附件、commit 附件、issue/PR 文件等。

## 子命令

### asset list

```
cnb-rs repo asset list [<repo>] [flags]
```

列出仓库的资产记录。

- `[<repo>]`: 仓库路径（如 `org/repo`），可选
- `-L, --limit <N>`: 最大列出数量

### asset delete

```
cnb-rs repo asset delete <id> [flags]
```

删除指定的仓库资产。仅 `slug_img` 和 `slug_file` 类型可删除。

- `<id>`: 资产 ID（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出（适用于 list）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出仓库资产
$ cnb-rs repo asset list
ID          TYPE          SIZE       PATH
ast-001     slug_img      1.2 MB     /images/logo.png
ast-002     repo_release  5.4 MB     /releases/v1.0/app.zip

# 限制数量
$ cnb-rs repo asset list --limit 10

# 删除指定资产
$ cnb-rs repo asset delete ast-001
确认删除资产 ast-001 ？ (yes/no): yes
✓ 资产已删除 (ast-001)

# 跳过确认
$ cnb-rs repo asset delete ast-001 --yes

# JSON 输出
$ cnb-rs repo asset list --json
```

## API

| 子操作   | API                                        | 方法   | 说明     |
| -------- | ------------------------------------------ | ------ | -------- |
| `list`   | `${CNB_API_ENDPOINT}/{slug}/-/list-assets` | GET    | 列出资产 |
| `delete` | `${CNB_API_ENDPOINT}/{repo}/-/assets/{id}` | DELETE | 删除资产 |

**API 详情**（OpenAPI）：

- [`ListAssets`](https://api.cnb.cool/#/operations/ListAssets) — 权限：`repo-manage:r`
- [`DeleteAsset`](https://api.cnb.cool/#/operations/DeleteAsset) — 权限：`repo-manage:rw`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs release asset-stats](/release/asset-stats)
