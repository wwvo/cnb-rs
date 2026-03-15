# cnb-rs stats

```
cnb-rs stats
```

显示仓库的提交统计仪表盘（TUI 界面）。

基于本地 `git log` 数据，使用 [ratatui](https://github.com/ratatui/ratatui) 渲染终端界面，包含：

- **左侧：历史提交排行榜** — 按用户统计提交次数，最多显示 13 位贡献者
- **右侧：提交趋势折线图** — 过去 80 周每周提交数量变化曲线

按 `q` 或 `Ctrl+C` 退出。

## 示例

```bash
$ cnb-rs stats
```

## 另请参阅

- [cnb-rs](/guide/cnb)
