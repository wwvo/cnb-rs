# cnb-rs issue view

```
cnb-rs issue view <NUMBER> [flags]
```

查看 Issue 详情。

显示 Issue 的完整信息，包括编号、标题、状态、优先级、作者、处理人、标签、
日期信息、评论数以及正文内容。

支持在浏览器中打开 Issue 页面。

## 选项

- `<NUMBER>`: Issue 编号（必填）
- `-w, --web`: 在浏览器中打开 Issue 页面

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看 Issue 详情
$ cnb-rs issue view 42
字段          值
编号          #42
标题          修复登录页面样式问题
状态          open
优先级        P1
作者          张三 (zhangsan)
处理人        李四 (lisi), 王五 (wangwu)
标签          bug, frontend
创建时间      2025-01-15T10:30:00Z
评论数        3

页面样式在移动端显示异常...

# 在浏览器中打开
$ cnb-rs issue view 42 --web
ℹ 正在打开 https://cnb.cool/org/repo/-/issues/42

# JSON 格式
$ cnb-rs --json issue view 42
```

## API

| 步骤       | API                                     | 方法 | 说明            |
| ---------- | --------------------------------------- | ---- | --------------- |
| 获取 Issue | `${API}/repos/{repo}/-/issues/{number}` | GET  | 获取 Issue 详情 |

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue edit](/issue/edit)
- [cnb-rs issue list](/issue/list)
