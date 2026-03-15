# cnb-rs repo transfer

```
cnb-rs repo transfer <repo> --target <group> [flags]
```

将仓库转移到另一个组织。

默认需要交互确认，可通过 `--yes` 跳过。

::: warning
转移后仓库的路径会变更为 `{target}/{repo_name}`，请确保相关引用已更新。
:::

## 选项

- `<repo>`: 仓库路径（如 `org-a/repo`，必填）
- `-t, --target <GROUP>`: 目标组织（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 转移仓库（交互确认）
$ cnb-rs repo transfer org-a/my-repo --target org-b
确定将 org-a/my-repo 转移到 org-b ？ (yes/no): yes
✓ 仓库已转移到 org-b/my-repo

# 跳过确认
$ cnb-rs repo transfer org-a/my-repo --target org-b --yes
```

## API

| 步骤     | API                                     | 方法 | 说明     |
| -------- | --------------------------------------- | ---- | -------- |
| 转移仓库 | `${CNB_API_ENDPOINT}/{repo}/-/transfer` | POST | 转移仓库 |

**API 详情**（OpenAPI：[`TransferRepo`](https://api.cnb.cool/#/operations/TransferRepo)）：

- **权限要求：** `repo-manage:rw,repo-code:rw`
- **请求体：**

```json
{
  "source": "org-a/my-repo",
  "target": "org-b"
}
```

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo delete](/repo/delete)
