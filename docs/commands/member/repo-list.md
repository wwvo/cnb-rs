---
title: cnb-rs member repo-list
---

# cnb-rs member repo-list

```
cnb-rs member repo-list [options]
```

列出仓库直接成员。

输出为表格格式，包含用户名、昵称、权限等级等信息。

## 选项

- `-r, --role <ROLE>`: 按角色过滤（如 `Developer`、`Master`）
- `-s, --search <KEYWORD>`: 搜索成员

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出仓库直接成员
$ cnb-rs member repo-list

# 按角色过滤
$ cnb-rs member repo-list --role Developer

# 搜索成员
$ cnb-rs member repo-list --search zhang

# JSON 格式输出
$ cnb-rs member repo-list --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-add](/member/repo-add)
- [cnb-rs member repo-all](/member/repo-all)
