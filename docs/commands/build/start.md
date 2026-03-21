---
title: cnb-rs build start
---

# cnb-rs build start

```
cnb-rs build start [options]
```

触发构建。默认触发主分支构建。

触发优先级：`sha` > `tag` > `branch` > 默认主分支。

## 选项

- `-b, --branch <BRANCH>`: 触发分支
- `-t, --tag <TAG>`: 触发 tag（优先级高于 branch）
- `--sha <SHA>`: Commit ID（优先级高于 tag）
- `-e, --event <EVENT>`: 事件名（默认：`api_trigger`）
- `--config <YAML>`: 配置文件内容（YAML）
- `--env <KEY=VALUE>`: 环境变量（可多次指定）
- `--sync`: 等待构建正式触发再返回

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 触发默认分支构建
$ cnb-rs build start

# 指定分支和事件
$ cnb-rs build start -b develop -e "api_trigger_deploy"

# 指定 tag 构建
$ cnb-rs build start -t v1.0.0

# 传入环境变量
$ cnb-rs build start --env "DEPLOY_ENV=production" --env "VERSION=1.0.0"

# 同步等待触发
$ cnb-rs build start --sync
```

## 另请参阅

- [cnb-rs build](/build/)
- [cnb-rs build stop](/build/stop)
- [cnb-rs build status](/build/status)
