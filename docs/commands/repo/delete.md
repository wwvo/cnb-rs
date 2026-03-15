# cnb-rs repo delete

```
cnb-rs repo delete <repo> [flags]
```

删除指定仓库。

默认需要交互确认（输入 `yes`），可通过 `--yes` 跳过。

::: warning
此操作不可逆，删除后仓库及其所有数据将无法恢复。
:::

## 选项

- `<repo>`: 仓库路径（如 `org/repo`，必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除仓库（交互确认）
$ cnb-rs repo delete org/old-repo
确定删除 org/old-repo ？此操作不可逆 (yes/no): yes
✓ 仓库 org/old-repo 已删除

# 跳过确认
$ cnb-rs repo delete org/old-repo --yes
✓ 仓库 org/old-repo 已删除

# 取消删除
$ cnb-rs repo delete org/old-repo
确定删除 org/old-repo ？此操作不可逆 (yes/no): no
已取消
```

## 错误处理

| 错误场景   | 说明               |
| ---------- | ------------------ |
| 仓库不存在 | 404 → "仓库不存在" |
| 权限不足   | 401 → 认证错误提示 |
| 用户取消   | 退出，不调用 API   |

## API

| 步骤     | API                          | 方法   | 说明     |
| -------- | ---------------------------- | ------ | -------- |
| 删除仓库 | `${CNB_API_ENDPOINT}/{repo}` | DELETE | 删除仓库 |

**API 详情**（OpenAPI：[`DeleteRepo`](https://api.cnb.cool/#/operations/DeleteRepo)）：

- **权限要求：** `repo-delete:rw`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo create](/repo/create)
