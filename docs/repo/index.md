# cnb repo

```
cnb repo <subcommand>
```

仓库管理，管理 CNB 平台仓库的创建、查看、编辑、删除等操作。

## 可用命令

- [cnb repo archive](/repo/archive) — 归档仓库
- [cnb repo asset](/repo/asset) — 管理仓库资产
- [cnb repo clone](/repo/clone) — 克隆仓库到本地
- [cnb repo contributor](/repo/contributor) — 查看贡献者趋势
- [cnb repo create](/repo/create) — 创建新仓库
- [cnb repo delete](/repo/delete) — 删除仓库
- [cnb repo edit](/repo/edit) — 编辑仓库信息
- [cnb repo events](/repo/events) — 获取仓库动态
- [cnb repo fork](/repo/fork) — 查看 Fork 列表
- [cnb repo list](/repo/list) — 列出仓库
- [cnb repo pin](/repo/pin) — 管理仓库墙（置顶仓库）
- [cnb repo security](/repo/security) — 查看安全概览
- [cnb repo top-contributors](/repo/top-contributors) — 查看活跃用户排名
- [cnb repo transfer](/repo/transfer) — 转移仓库
- [cnb repo unarchive](/repo/unarchive) — 解除仓库归档
- [cnb repo view](/repo/view) — 查看仓库详情
- [cnb repo visibility](/repo/visibility) — 修改仓库可见性

## 仓库路径

许多子命令接受可选的 `<repo>` 参数来指定仓库路径（格式：`org/repo`）。

如果不指定，CLI 会自动从当前 Git 目录的 remote URL 推断仓库路径。
也可以通过全局 `--repo` 参数覆盖：

```bash
cnb --repo org/repo repo view
```

## 另请参阅

- [cnb](/guide/cnb)
- [cnb auth](/auth/)
