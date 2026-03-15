# cnb-rs repo archive

```
cnb-rs repo archive <repo> [flags]
```

归档仓库，使其变为只读状态。

默认需要交互确认，可通过 `--yes` 跳过。

## 选项

- `<repo>`: 仓库路径（如 `org/repo`，必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 归档仓库（交互确认）
$ cnb-rs repo archive org/old-project
确定归档 org/old-project ？归档后仓库将变为只读 (yes/no): yes
✓ 仓库已归档 (org/old-project)

# 跳过确认
$ cnb-rs repo archive org/old-project --yes
```

## API

| 步骤     | API                                             | 方法 | 说明     |
| -------- | ----------------------------------------------- | ---- | -------- |
| 归档仓库 | `${CNB_API_ENDPOINT}/{slug}/-/settings/archive` | POST | 归档仓库 |

**API 详情**（OpenAPI：[`ArchiveRepo`](https://api.cnb.cool/#/operations/ArchiveRepo)）：

- **权限要求：** `repo-manage:rw,repo-code:rw`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo unarchive](/repo/unarchive)
