# cnb-rs repo create

```
cnb-rs repo create <name> [flags]
```

在指定组织下创建新仓库。

不指定 `--group` 时，默认创建到当前认证用户的个人空间。
默认创建公开仓库，可通过 `--private` 或 `--secret` 切换可见性。

## 选项

- `<name>`: 仓库名称（必填）
- `-g, --group <GROUP>`: 所属组织（不指定则创建到个人空间）
- `-d, --description <TEXT>`: 仓库描述
- `-l, --license <LICENSE>`: 开源许可证
- `--private`: 创建私有仓库
- `--secret`: 创建加密仓库

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 在组织下创建公开仓库
$ cnb-rs repo create my-project --group my-org
✓ 仓库已创建：https://cnb.cool/my-org/my-project

# 创建私有仓库并添加描述
$ cnb-rs repo create my-lib --group my-org --private --description "内部库"
✓ 仓库已创建：https://cnb.cool/my-org/my-lib

# 创建加密仓库
$ cnb-rs repo create my-secret --group my-org --secret

# 创建到个人空间
$ cnb-rs repo create my-app
✓ 仓库已创建：https://cnb.cool/octocat/my-app
```

## API

| 步骤     | API                                  | 方法 | 说明     |
| -------- | ------------------------------------ | ---- | -------- |
| 创建仓库 | `${CNB_API_ENDPOINT}/{slug}/-/repos` | POST | 创建仓库 |

**API 详情**（OpenAPI：[`CreateRepo`](https://api.cnb.cool/#/operations/CreateRepo)）：

- **权限要求：** `group-resource:rw`
- **请求体：**

```json
{
  "name": "string",
  "description": "string",
  "license": "string",
  "visibility": "public | private | secret"
}
```

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo view](/repo/view)
- [cnb-rs repo delete](/repo/delete)
