---
title: cnb-rs release update
---

# cnb-rs release update

```
cnb-rs release update <TAG> [options]
```

更新 Release 信息。至少需要指定一项修改内容。

## 选项

- `<TAG>`: Tag 名称（必填）
- `-n, --name <NAME>`: 修改 Release 名称
- `-b, --body <BODY>`: 修改 Release 描述
- `--draft`: 标记为草稿
- `--prerelease`: 标记为预发布
- `--make-latest <VALUE>`: 设置最新版本标记：`true`、`false`、`legacy`

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 更新描述
$ cnb-rs release update v1.2.0 -b "新的发布说明"

# 修改名称
$ cnb-rs release update v1.2.0 -n "v1.2.0 正式版"

# 标记为预发布
$ cnb-rs release update v1.2.0 --prerelease

# 设置为最新版本
$ cnb-rs release update v1.2.0 --make-latest true
```

## API

| 方法  | 端点                              |
| ----- | --------------------------------- |
| PATCH | `/{repo}/-/releases/{release_id}` |

## 另请参阅

- [cnb-rs release](/release/)
- [cnb-rs release view](/release/view)
