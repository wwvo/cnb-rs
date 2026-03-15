# cnb-rs config

配置管理，查看或修改 CNB CLI 的配置项。

当前支持的配置项：

- `domain` — CNB 平台域名（默认：`cnb.cool`）
- `git_protocol` — Git 协议偏好 {`https` | `ssh`}（默认：`https`，CNB 暂不支持 SSH 克隆）

配置文件路径：`~/.cnb/config.toml`

## 可用命令

- [cnb-rs config get](/config/get) — 获取配置项的值
- [cnb-rs config list](/config/list) — 列出所有配置项
- [cnb-rs config set](/config/set) — 设置配置项的值

## 另请参阅

- [cnb-rs](/guide/cnb)
