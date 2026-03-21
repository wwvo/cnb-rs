---
title: cnb-rs member repo-inherited
---

# cnb-rs member repo-inherited

```
cnb-rs member repo-inherited [options]
```

列出仓库继承成员。

继承成员是通过上级组织权限自动获得仓库访问权限的用户。

## 选项

- `-r, --role <ROLE>`: 按角色过滤
- `-s, --search <KEYWORD>`: 搜索成员

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出仓库继承成员
$ cnb-rs member repo-inherited

# 按角色过滤
$ cnb-rs member repo-inherited --role Master

# JSON 格式输出
$ cnb-rs member repo-inherited --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-list](/member/repo-list)
- [cnb-rs member repo-all](/member/repo-all)
