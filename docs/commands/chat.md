# cnb-rs chat

```
cnb-rs chat [flags]
```

使用自然语言与 CNB OpenAPI 交互。

内置 Agent 循环，可理解自然语言请求、自动查询 API 文档、构造并执行 API 调用，将结果以可读方式返回。

默认进入交互式 REPL 模式（基于 rustyline），支持行编辑和历史记录。使用 `--do` 参数可执行单次请求后退出，适合脚本和 CI 场景。

## 选项

- `--do <问题>`: 一次性模式，执行单个请求后退出。不提供此参数则进入交互式 REPL
- `--no-stream`: 禁用流式输出，等待完整响应后一次性输出（适合 CI 环境或管道）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 交互式模式

进入交互式模式后，提示符为 `>>>`。支持以下操作：

- **上/下箭头** — 浏览历史记录
- **Ctrl+C**（输入中）— 忽略当前输入，继续等待
- **Ctrl+C**（执行中）— 中断当前请求
- **Ctrl+D** — 退出
- `/exit`、`/bye`、`exit`、`bye` — 退出

历史记录自动保存在 `~/.local/share/cnb/chat_history.txt`（Windows 上对应 `%LOCALAPPDATA%\cnb\chat_history.txt`）。

## 示例

```bash
# 交互式模式
$ cnb-rs chat

# 一次性模式
$ cnb-rs chat --do "查看我的 Issue 列表"

# 禁用流式输出
$ cnb-rs chat --do "查看仓库信息" --no-stream
```

## 另请参阅

- [cnb-rs auth](/auth/) — 登录后才能使用 chat
- [cnb-rs config](/config/) — 配置域名等信息
