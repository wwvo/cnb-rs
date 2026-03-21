---
title: cnb-rs workspace
---

# cnb-rs workspace

```
cnb-rs workspace <subcommand>
```

管理云原生工作区。

支持启动、停止、删除、查看和清理工作区等操作。
云原生工作区是 CNB 平台提供的在线开发环境，支持 WebIDE、VS Code、Cursor 等多种 IDE 接入。

## 可用命令

- [cnb-rs workspace list](/workspace/list) — 列出我的工作区
- [cnb-rs workspace start](/workspace/start) — 启动工作区
- [cnb-rs workspace stop](/workspace/stop) — 停止工作区
- [cnb-rs workspace delete](/workspace/delete) — 删除工作区
- [cnb-rs workspace detail](/workspace/detail) — 查看工作区详情
- [cnb-rs workspace closed-clean](/workspace/closed-clean) — 清理已关闭的工作区

## 示例

```bash
# 列出所有工作区
$ cnb-rs workspace list

# 启动当前仓库的开发环境
$ cnb-rs workspace start

# 停止工作区
$ cnb-rs workspace stop -p <pipeline-id>

# 查看工作区详情
$ cnb-rs workspace detail --sn <sn>

# 清理已关闭的工作区
$ cnb-rs workspace closed-clean
```

## 另请参阅

- [cnb-rs](/cnb)
