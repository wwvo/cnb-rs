# cnb-rs build crontab-sync

```
cnb-rs build crontab-sync [branch]
```

同步指定分支的定时任务配置。

将仓库配置文件中定义的定时任务同步到 CI/CD 引擎。不指定分支时默认同步主分支。

## 选项

- `[branch]`: 分支名（默认：`main`）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 同步默认分支的定时任务
$ cnb-rs build crontab-sync
✓ 分支 main 的定时任务已同步

# 指定分支
$ cnb-rs build crontab-sync develop
✓ 分支 develop 的定时任务已同步
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build start](/build/start)
