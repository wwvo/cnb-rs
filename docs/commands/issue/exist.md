# cnb-rs issue exist

```
cnb-rs issue exist <NUMBER>
```

检查指定编号的 Issue 是否存在。

如果 Issue 存在，输出其标题；如果不存在，输出 `false`。
适合在脚本中用于条件判断。

## 选项

- `<NUMBER>`: Issue 编号（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# Issue 存在
$ cnb-rs issue exist 123
修复登录页面样式问题

# Issue 不存在
$ cnb-rs issue exist 99999
false

# 在脚本中使用
$ if [ "$(cnb-rs issue exist 123)" != "false" ]; then echo "Issue 存在"; fi
```

## API

| 步骤       | API                                     | 方法 | 说明            |
| ---------- | --------------------------------------- | ---- | --------------- |
| 获取 Issue | `${API}/repos/{repo}/-/issues/{number}` | GET  | 获取 Issue 详情 |

返回 `404` 时输出 `false`，否则输出 Issue 标题。

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue view](/issue/view)
