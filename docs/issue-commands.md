# cnb issue 命令参考

## 概览

`cnb issue` 提供 Issue 的完整生命周期管理，包括列表查询、创建、关闭、评论、下载和处理人管理。

```
cnb issue <COMMAND>

Commands:
  list        列出仓库的 Issue
  mine        列出与我相关的 Issue
  create      创建 Issue
  close       关闭 Issue
  comment     创建 Issue 评论
  exist       检查 Issue 是否存在
  download    下载 Issue 为 Markdown 文件
  assigners   Issue 处理人管理（获取/添加）
```

---

## cnb issue list

列出当前仓库所有 open 状态的 Issue，支持按不活跃天数过滤。

```bash
# 列出所有 open 的 Issue
cnb issue list

# 列出 30 天内没有活动的 Issue
cnb issue list --stale-days 30

# 指定仓库
cnb --repo looc/git-cnb issue list
```

**参数：**

| 参数 | 短名 | 类型 | 默认值 | 说明 |
|------|------|------|--------|------|
| `--stale-days` | `-d` | int | 0 | 过滤 N 天内没有活动的 Issue（0 表示不过滤） |

**输出示例：**

```
NUMBER     TITLE                                                        LastActedAt               StaleDays
42         修复登录页面样式问题                                           2026-02-01T10:00:00Z      32
```

---

## cnb issue mine

列出与当前用户相关的 Issue，包括分配给我的和我创建的，自动标记关系类型。

```bash
cnb issue mine
```

**输出说明：**

| TYPE | 含义 |
|------|------|
| `->ME` | Issue 分配给我 |
| `ME->` | Issue 由我创建 |
| `ME->ME` | Issue 由我创建且分配给我 |

额外功能：自动查询同组织下 `feedback` 仓库中与我相关的 Issue。

---

## cnb issue create

创建一个新的 Issue。

```bash
# 仅指定标题
cnb issue create --title "修复登录问题"

# 指定标题和描述
cnb issue create -t "修复登录问题" -b "详细描述..."
```

**参数：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--title` | `-t` | string | 是 | Issue 标题 |
| `--body` | `-b` | string | 否 | Issue 描述 |

**输出：** 创建成功后输出 Issue 的 Web 链接。

```
https://cnb.cool/looc/git-cnb/-/issues/42
```

---

## cnb issue close

关闭指定的 Issue。

```bash
cnb issue close --number 42
cnb issue close -n 42
```

**参数：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--number` | `-n` | string | 是 | Issue 编号 |

关闭原因默认为 `not_planned`。

---

## cnb issue comment

为指定 Issue 创建评论。

```bash
cnb issue comment --number 42 --comment "已修复，请验证"
cnb issue comment -n 42 -c "已修复，请验证"
```

**参数：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--number` | `-n` | string | 是 | Issue 编号 |
| `--comment` | `-c` | string | 是 | 评论内容 |

---

## cnb issue exist

检查 Issue 是否存在。存在时输出标题，不存在时输出 `false`。适用于脚本化场景。

```bash
cnb issue exist --number 42
cnb issue exist -n 42
```

**参数：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--number` | `-n` | string | 是 | Issue 编号 |

**输出示例：**

```
修复登录页面样式问题
```

或

```
false
```

---

## cnb issue download

将 Issue 内容和评论下载为 Markdown 文件，保存到当前目录的 `.issues/` 文件夹中。

```bash
# 下载单个 Issue
cnb issue download --number 42

# 下载全部 Issue（open + closed）
cnb issue download --all
```

**参数：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--number` | `-n` | string | 否 | 指定下载的 Issue 编号 |
| `--all` | | bool | 否 | 下载全部 Issue |

`--number` 和 `--all` 必须指定其一。

**输出文件格式：** `.issues/{number}.md`

```markdown
# Issue 标题

issue number: 42

Issue 描述内容...

## Comments

### Comment by 昵称 (用户名)

**Created:** 2026-03-01T10:00:00Z

评论内容...
```

---

## cnb issue assigners

管理 Issue 的处理人，支持获取和添加。

### 获取处理人

```bash
cnb issue assigners get --number 42
cnb issue assigners get -n 42
```

### 添加处理人

```bash
# 添加单个处理人
cnb issue assigners add --number 42 --assignees zhangsan

# 添加多个处理人（逗号分隔，自动去重）
cnb issue assigners add -n 42 -a "zhangsan,lisi,wangwu"
```

**参数（get）：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--number` | `-n` | string | 是 | Issue 编号 |

**参数（add）：**

| 参数 | 短名 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `--number` | `-n` | string | 是 | Issue 编号 |
| `--assignees` | `-a` | string | 是 | 处理人用户名，多个用逗号分隔 |

---

## 全局参数

以下参数适用于所有 `cnb issue` 子命令：

| 参数 | 类型 | 说明 |
|------|------|------|
| `--domain` | string | 指定 CNB 域名（默认 cnb.cool） |
| `--repo` | string | 指定仓库路径（如 looc/git-cnb） |

## 认证

所有 Issue 命令需要有效的 CNB Token：

```bash
export CNB_TOKEN="your_token"
```

Token 权限要求：`repo-notes:rw`
