---
title: cnb-rs issue mine
---

# cnb-rs issue mine

```
cnb-rs issue mine
```

列出与我相关的 Issue。

查询当前仓库中分配给我的和我创建的所有 `open` 状态 Issue，合并去重后输出。
每条 Issue 通过 `TYPE` 列标注关系类型：

- **`->ME`** — 分配给我（我是处理人）
- **`ME->`** — 我创建的
- **`ME->ME`** — 我创建且分配给我

此外，还会尝试查询同组织下 `feedback` 仓库中分配给我的 Issue，一并展示。

## 选项

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 列出与我相关的 Issue
$ cnb-rs issue mine
NUMBER          TITLE                                                            TYPE
42              修复登录页面样式问题                                              ->ME
58              新增导出功能                                                      ME->
73              重构用户模块                                                      ME->ME

# JSON 格式输出
$ cnb-rs --json issue mine

# 没有相关 Issue 时
$ cnb-rs issue mine
ℹ 没有找到与我相关的 Issue
```

## API

| 步骤             | API                                      | 方法 | 说明                         |
| ---------------- | ---------------------------------------- | ---- | ---------------------------- |
| 获取当前用户     | `${API}/user`                            | GET  | 获取当前登录用户名           |
| 分配给我的 Issue | `${API}/repos/{repo}/-/issues`           | GET  | `assignees={username}`       |
| 我创建的 Issue   | `${API}/repos/{repo}/-/issues`           | GET  | `authors={username}`         |
| feedback 仓库    | `${API}/repos/{group}/feedback/-/issues` | GET  | 额外查询同组织 feedback 仓库 |

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue list](/issue/list)
