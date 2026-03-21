---
title: cnb-rs workspace start
---

# cnb-rs workspace start

```
cnb-rs workspace start [options]
```

启动云原生开发环境。

如果已有环境则直接打开，否则创建新的开发环境。

## 选项

- `-b, --branch <BRANCH>`: 分支名（默认从当前 Git 分支推断，未检测到则使用 `main`）
- `-t, --tag <TAG>`: Tag 名称
- `--open`: 自动在浏览器中打开

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 启动当前仓库的开发环境
$ cnb-rs workspace start

# 指定分支
$ cnb-rs workspace start -b feature/new-ui

# 指定 Tag
$ cnb-rs workspace start -t v1.0.0

# 启动并自动打开浏览器
$ cnb-rs workspace start --open

# JSON 格式输出
$ cnb-rs workspace start --json
```

## 另请参阅

- [cnb-rs workspace](/workspace/)
- [cnb-rs workspace stop](/workspace/stop)
- [cnb-rs workspace detail](/workspace/detail)
