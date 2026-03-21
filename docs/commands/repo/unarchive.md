---
title: cnb-rs repo unarchive
---

# cnb-rs repo unarchive

```
cnb-rs repo unarchive <repo>
```

解除仓库归档，恢复可写状态。

## 选项

- `<repo>`: 仓库路径（如 `org/repo`，必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs repo unarchive org/old-project
✓ 仓库已解除归档 (org/old-project)
```

## API

| 步骤     | API                                               | 方法 | 说明         |
| -------- | ------------------------------------------------- | ---- | ------------ |
| 解除归档 | `${CNB_API_ENDPOINT}/{slug}/-/settings/unarchive` | POST | 解除仓库归档 |

**API 详情**（OpenAPI：[`UnArchiveRepo`](https://api.cnb.cool/#/operations/UnArchiveRepo)）：

- **权限要求：** `repo-manage:rw,repo-code:rw`

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo archive](/repo/archive)
