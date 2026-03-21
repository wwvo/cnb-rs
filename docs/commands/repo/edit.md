---
title: cnb-rs repo edit
---

# cnb-rs repo edit

```
cnb-rs repo edit [<repo>] [flags]
```

编辑仓库信息。可更新描述、许可证、站点 URL 和主题标签。

不指定仓库路径时，使用当前 Git 目录对应的仓库。
至少需要指定一个要修改的字段，否则命令会报错。

## 选项

- `[<repo>]`: 仓库路径（如 `org/repo`），可选
- `-d, --description <TEXT>`: 仓库描述
- `-l, --license <LICENSE>`: 开源许可证
- `-s, --site <URL>`: 仓库站点 URL
- `-t, --topics <T1,T2,...>`: 主题标签（逗号分隔）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 更新当前仓库的描述
$ cnb-rs repo edit --description "新的描述"
✓ 仓库信息已更新 (org/repo)

# 更新指定仓库的许可证和主题
$ cnb-rs repo edit org/repo --license MIT --topics "cli,rust,tool"
✓ 仓库信息已更新 (org/repo)

# 更新站点 URL
$ cnb-rs repo edit --site "https://example.com"
```

## API

| 步骤     | API                          | 方法  | 说明         |
| -------- | ---------------------------- | ----- | ------------ |
| 更新仓库 | `${CNB_API_ENDPOINT}/{repo}` | PATCH | 更新仓库信息 |

**API 详情**（OpenAPI：[`UpdateRepo`](https://api.cnb.cool/#/operations/UpdateRepo)）：

- **权限要求：** `repo-manage:rw`
- **请求体：**

```json
{
  "description": "string",
  "license": "string",
  "site": "string",
  "topics": ["string"]
}
```

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs repo view](/repo/view)
