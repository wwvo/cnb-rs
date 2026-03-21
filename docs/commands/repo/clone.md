---
title: cnb-rs repo clone
---

# cnb-rs repo clone

```
cnb-rs repo clone <repo> [flags]
```

克隆仓库到本地。封装 `git clone`，自动根据配置的 Git 协议构造 CNB 仓库 URL。

## 选项

- `<repo>`: 仓库路径（如 `org/repo`，必填）
- `--dir <DIR>`: 克隆到指定目录
- `--depth <N>`: 浅克隆深度

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 克隆 URL

根据 `git_protocol` 配置自动选择：

| 协议    | URL 格式                              |
| ------- | ------------------------------------- |
| `https` | `https://{domain}/{repo}.git`（默认） |
| `ssh`   | `git@{domain}:{repo}.git`             |

可通过 `cnb-rs config set git_protocol ssh` 切换协议。

> **注意**：CNB 暂不支持 SSH 克隆，当前仅 HTTPS 协议可用。

## 示例

```bash
# 克隆仓库
$ cnb-rs repo clone org/my-project
✓ 仓库已克隆到 ./my-project

# 浅克隆
$ cnb-rs repo clone org/my-project --depth 1

# 克隆到指定目录
$ cnb-rs repo clone org/my-project --dir ~/projects/my-project
```

## 另请参阅

- [cnb-rs repo](/repo/)
- [cnb-rs config](/config/)
