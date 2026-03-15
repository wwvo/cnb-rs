# cnb-rs mission

```
cnb-rs mission <subcommand>
```

管理组织下的任务集（Mission）。

任务集是 CNB 平台中跨仓库的项目管理工具，支持将多个仓库的 Issue 聚合到一个看板中统一管理。
每个任务集可配置多个视图（看板、列表等），用于不同维度的任务展示和管理。

## 可用命令

### 任务集基础管理

- [cnb-rs mission list](/mission/list) — 列出组织下的任务集
- [cnb-rs mission create](/mission/create) — 创建任务集
- [cnb-rs mission delete](/mission/delete) — 删除任务集
- [cnb-rs mission set-visibility](/mission/set-visibility) — 设置任务集可见性

### 视图管理

- [cnb-rs mission view list](/mission/view-list) — 列出视图
- [cnb-rs mission view get](/mission/view-get) — 获取视图配置
- [cnb-rs mission view set](/mission/view-set) — 设置视图配置
- [cnb-rs mission view add](/mission/view-add) — 添加/修改视图
- [cnb-rs mission view sort](/mission/view-sort) — 排序视图

## 示例

```bash
# 列出组织下的任务集
$ cnb-rs mission list --group my-org

# 创建任务集
$ cnb-rs mission create --group my-org --name "Sprint 2025-Q2"

# 删除任务集
$ cnb-rs mission delete my-org/sprint-2025-q1

# 列出视图
$ cnb-rs mission view list my-org/sprint-2025-q1
```

## 另请参阅

- [cnb-rs](/guide/cnb)
- [cnb-rs issue](/issue/)
