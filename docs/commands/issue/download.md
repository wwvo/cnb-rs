---
title: cnb-rs issue download
---

# cnb-rs issue download

```
cnb-rs issue download [NUMBER] [flags]
```

下载 Issue 为 Markdown 文件，保存到当前目录的 `.issues/` 子目录。

每个 Issue 生成一个 `{编号}.md` 文件，包含标题、编号、描述正文和所有评论。
评论按时间顺序排列，包含作者、创建时间和更新时间。

必须指定 Issue 编号或使用 `--all` 下载全部。

## 选项

- `[NUMBER]`: 指定下载的 Issue 编号（与 `--all` 二选一）
- `--all`: 下载所有 Issue（同时获取 `open` 和 `closed` 状态，自动分页）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 输出格式

生成的 Markdown 文件结构如下：

```markdown
# Issue 标题

issue number: 42

Issue 描述正文...

## Comments

### Comment by 张三 (zhangsan)

**Created:** 2025-01-15T10:30:00Z

评论内容...
```

## 示例

```bash
# 下载单个 Issue
$ cnb-rs issue download 123
ℹ 已下载 Issue #123: 修复登录问题
ℹ 下载完成，共下载了 1 个 Issue

# 下载所有 Issue
$ cnb-rs issue download --all
ℹ 已下载 Issue #1: 初始化项目
ℹ 已下载 Issue #2: 添加 CI/CD
ℹ 已下载 Issue #3: 修复构建问题
ℹ 下载完成，共下载了 3 个 Issue
```

## API

| 步骤            | API                                              | 方法 | 说明             |
| --------------- | ------------------------------------------------ | ---- | ---------------- |
| 列出所有 Issue  | `${API}/repos/{repo}/-/issues`                   | GET  | 自动分页获取全部 |
| 获取 Issue 详情 | `${API}/repos/{repo}/-/issues/{number}`          | GET  | 获取标题和正文   |
| 获取 Issue 评论 | `${API}/repos/{repo}/-/issues/{number}/comments` | GET  | 自动分页获取全部 |

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue view](/issue/view)
