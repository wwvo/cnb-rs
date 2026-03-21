---
title: cnb-rs pr create
---

# cnb-rs pr create

```
cnb-rs pr create [flags]
```

创建 Pull Request。

默认从当前 Git 分支向远程仓库的 HEAD 分支发起 PR，标题取最新 commit 信息。所有参数均可通过选项覆盖。

创建成功后输出 PR 的 Web 链接。

## 选项

- `-B, --base-branch <BRANCH>`: 目标分支（默认：远程仓库的 HEAD 分支）
- `-H, --head-branch <BRANCH>`: 源分支（默认：当前 Git 分支）
- `-R, --remote-repo <OWNER/REPO>`: 目标仓库路径（默认：当前仓库，用于跨仓库 PR）
- `--title <TITLE>`: PR 标题（默认：最新 commit 的标题）
- `--body <BODY>`: PR 描述（默认：空）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 从当前分支向默认分支发起 PR
$ cnb-rs pr create

# 指定目标分支
$ cnb-rs pr create --base-branch main

# 指定标题和描述
$ cnb-rs pr create --title "feat: 新增登录功能" --body "详细描述..."

# 跨仓库 PR
$ cnb-rs pr create -R upstream/repo -B main -H feature-branch
```

## 另请参阅

- [cnb-rs pr](/pr/)
