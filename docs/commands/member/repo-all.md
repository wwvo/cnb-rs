---
title: cnb-rs member repo-all
---

# cnb-rs member repo-all

```
cnb-rs member repo-all [options]
```

列出仓库所有有效成员（含继承）。

合并直接成员和继承成员，展示仓库中所有拥有访问权限的用户。

## 选项

- `-r, --role <ROLE>`: 按角色过滤
- `-s, --search <KEYWORD>`: 搜索成员
- `--names <NAMES>`: 精准匹配用户名（逗号分隔）
- `--order-by <FIELD>`: 排序字段
- `--desc`: 降序排列

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出所有有效成员
$ cnb-rs member repo-all

# 按角色过滤并降序
$ cnb-rs member repo-all --role Developer --desc

# 精准匹配用户名
$ cnb-rs member repo-all --names "zhangsan,lisi"

# JSON 格式输出
$ cnb-rs member repo-all --json
```

## 另请参阅

- [cnb-rs member](/member/)
- [cnb-rs member repo-list](/member/repo-list)
- [cnb-rs member repo-inherited](/member/repo-inherited)
