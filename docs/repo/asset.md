# cnb repo asset

```
cnb repo asset <list|delete> [flags]
```

管理仓库资产记录，包括 release 附件、commit 附件、issue/PR 文件等。

## 子命令

### `cnb repo asset list`

```
cnb repo asset list [<repo>] [flags]
```

列出仓库的资产记录。

**选项：**

`<repo>`
: 仓库路径（如 `org/repo`），可选

`--limit <N>`, `-L`
: 最大列出数量

**继承的全局选项：**

`--json`
: 以 JSON 格式输出

### `cnb repo asset delete`

```
cnb repo asset delete <id> [flags]
```

删除指定的仓库资产。

::: warning
仅 `slug_img` 和 `slug_file` 类型的资产可通过此命令删除，`repo_release` 和 `repo_commit` 类型不可删除。
:::

**选项：**

`<id>`
: 资产 ID（必填）

`--repo <REPO>`
: 仓库路径（如 `org/repo`），可选

`--yes`, `-y`
: 跳过确认提示

## 示例

```bash
# 列出仓库资产
$ cnb repo asset list
ID          TYPE          SIZE       PATH
ast-001     slug_img      1.2 MB     /images/logo.png
ast-002     repo_release  5.4 MB     /releases/v1.0/app.zip

# 限制数量
$ cnb repo asset list --limit 10

# 删除指定资产
$ cnb repo asset delete ast-001
确认删除资产 ast-001 ？ (yes/no): yes
✓ 资产已删除 (ast-001)

# 跳过确认
$ cnb repo asset delete ast-001 --yes

# JSON 输出
$ cnb repo asset list --json
```

## API

| 子操作   | API                                          | 方法   | 说明     |
| -------- | -------------------------------------------- | ------ | -------- |
| `list`   | `${CNB_API_ENDPOINT}/{slug}/-/list-assets`   | GET    | 列出资产 |
| `delete` | `${CNB_API_ENDPOINT}/{repo}/-/assets/{id}`   | DELETE | 删除资产 |

**API 详情**（OpenAPI）：

- [`ListAssets`](https://api.cnb.cool/#/operations/ListAssets) — 权限：`repo-manage:r`
- [`DeleteAsset`](https://api.cnb.cool/#/operations/DeleteAsset) — 权限：`repo-manage:rw`

## 另请参阅

- [cnb repo](/repo/)
- [cnb release asset-stats](/release/asset-stats)
