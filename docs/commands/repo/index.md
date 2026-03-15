# cnb-rs repo

```
cnb-rs repo <subcommand>
```

仓库管理，管理 CNB 平台仓库的创建、查看、编辑、删除等操作。

## 可用命令

- [cnb-rs repo archive](/repo/archive) — 归档仓库
- [cnb-rs repo asset](/repo/asset) — 管理仓库资产
- [cnb-rs repo clone](/repo/clone) — 克隆仓库到本地
- [cnb-rs repo contributor](/repo/contributor) — 查看贡献者趋势
- [cnb-rs repo create](/repo/create) — 创建新仓库
- [cnb-rs repo delete](/repo/delete) — 删除仓库
- [cnb-rs repo edit](/repo/edit) — 编辑仓库信息
- [cnb-rs repo events](/repo/events) — 获取仓库动态
- [cnb-rs repo fork](/repo/fork) — 查看 Fork 列表
- [cnb-rs repo list](/repo/list) — 列出仓库
- [cnb-rs repo pin](/repo/pin) — 管理仓库墙（置顶仓库）
- [cnb-rs repo security](/repo/security) — 查看安全概览
- [cnb-rs repo settings](/repo/settings) — 仓库设置管理
- [cnb-rs repo top-contributors](/repo/top-contributors) — 查看活跃用户排名
- [cnb-rs repo transfer](/repo/transfer) — 转移仓库
- [cnb-rs repo unarchive](/repo/unarchive) — 解除仓库归档
- [cnb-rs repo view](/repo/view) — 查看仓库详情
- [cnb-rs repo visibility](/repo/visibility) — 修改仓库可见性

## 仓库路径

许多子命令接受可选的 `<repo>` 参数来指定仓库路径（格式：`org/repo`）。

如果不指定，CLI 会自动从当前 Git 目录的 remote URL 推断仓库路径。
也可以通过全局 `--repo` 参数覆盖：

```bash
cnb-rs --repo org/repo repo view
```

## 另请参阅

- [cnb-rs](/guide/cnb)
- [cnb-rs auth](/auth/)
