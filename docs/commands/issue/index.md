---
title: cnb-rs issue
---

# cnb-rs issue

```
cnb-rs issue <subcommand>
```

Issue 管理，查看、创建和管理仓库的 Issue。

支持 Issue 的完整生命周期管理：创建、查看、编辑、关闭、重新打开，
以及评论、标签、处理人等关联操作。

## 可用命令

- [cnb-rs issue list](/issue/list) — 列出仓库的 Issue
- [cnb-rs issue mine](/issue/mine) — 列出与我相关的 Issue
- [cnb-rs issue view](/issue/view) — 查看 Issue 详情
- [cnb-rs issue create](/issue/create) — 创建 Issue
- [cnb-rs issue edit](/issue/edit) — 编辑 Issue
- [cnb-rs issue close](/issue/close) — 关闭 Issue
- [cnb-rs issue reopen](/issue/reopen) — 重新打开 Issue
- [cnb-rs issue comment](/issue/comment) — 创建 Issue 评论
- [cnb-rs issue exist](/issue/exist) — 检查 Issue 是否存在
- [cnb-rs issue download](/issue/download) — 下载 Issue 为 Markdown 文件
- [cnb-rs issue assigners](/issue/assigners) — Issue 处理人管理
- [cnb-rs issue label](/issue/label) — Issue 标签管理

## 示例

```bash
# 列出当前仓库的 Issue
$ cnb-rs issue list

# 创建一个新 Issue
$ cnb-rs issue create -t "修复登录问题" -b "详细描述"

# 查看 Issue 详情
$ cnb-rs issue view 42

# 列出与我相关的 Issue
$ cnb-rs issue mine

# 指定仓库操作
$ cnb-rs --repo org/repo issue list

# JSON 格式输出
$ cnb-rs --json issue list
```

## 另请参阅

- [cnb-rs](/cnb)
