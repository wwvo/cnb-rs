---
title: cnb-rs build
---

# cnb-rs build

```
cnb-rs build <subcommand>
```

管理仓库的 CI/CD 构建流水线。

支持触发构建、停止构建、查询状态、查看日志等操作。
构建由 CNB 平台的 CI/CD 引擎驱动，支持多流水线并行执行。

## 可用命令

- [cnb-rs build start](/build/start) — 触发构建
- [cnb-rs build stop](/build/stop) — 停止构建
- [cnb-rs build status](/build/status) — 查询构建状态
- [cnb-rs build list](/build/list) — 列出构建记录
- [cnb-rs build stage](/build/stage) — 查看 Stage 详情
- [cnb-rs build download-log](/build/download-log) — 下载 Runner 日志
- [cnb-rs build delete-log](/build/delete-log) — 删除构建日志
- [cnb-rs build crontab-sync](/build/crontab-sync) — 同步定时任务

## 示例

```bash
# 触发默认分支构建
$ cnb-rs build start

# 查看构建列表
$ cnb-rs build list

# 查询构建状态
$ cnb-rs build status cnb-1qa-1i3f5ecau

# 停止构建
$ cnb-rs build stop cnb-1qa-1i3f5ecau
```

## 另请参阅

- [cnb-rs](/cnb)
- [cnb-rs repo](/repo/)
