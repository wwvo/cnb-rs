# cnb-rs repo visibility

```
cnb-rs repo visibility <repo> <visibility> [flags]
```

修改仓库的可见性级别。

默认需要交互确认，可通过 `--yes` 跳过。

## 选项

- `<repo>`: 仓库路径（如 `org/repo`，必填）
- `<visibility>`: 目标可见性（必填），可选值：`public`、`private`、`secret`
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 设为公开
$ cnb-rs repo visibility org/repo public
确定将 org/repo 的可见性修改为 public ？ (yes/no): yes
✓ 仓库可见性已修改为 public (org/repo)

# 设为私有（跳过确认）
$ cnb-rs repo visibility org/repo private --yes
✓ 仓库可见性已修改为 private (org/repo)
```

## API

| 步骤       | API                                                    | 方法 | 说明       |
| ---------- | ------------------------------------------------------ | ---- | ---------- |
| 设置可见性 | `${CNB_API_ENDPOINT}/{repo}/-/settings/set_visibility` | POST | 修改可见性 |

**API 详情**（OpenAPI：[`SetRepoVisibility`](https://api.cnb.cool/#/operations/SetRepoVisibility)）：

- **权限要求：** `repo-manage:rw`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo edit](/repo/edit)
