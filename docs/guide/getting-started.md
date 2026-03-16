# 快速开始

## 安装

从 [Release 页面](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/releases) 下载适合你平台的二进制文件。

> [!WARNING]
> 从改名版本开始，原来的 `cnb ...` 已改为 `cnb-rs ...`。
> 如果你是从旧版本升级，请先阅读 [从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)。

## 登录

```bash
cnb-rs auth login
# 或直接指定 Token
cnb-rs auth login --token <YOUR_TOKEN>
```

## 基本使用

```bash
# 查看当前认证状态
cnb-rs auth status

# 查看仓库信息
cnb-rs info

# 查看 Issue 列表
cnb-rs issue list

# 使用 AI 对话
cnb-rs chat --do "查看我的 Issue 列表"
```

## 升级自旧版本

- 旧命令：`cnb ...`
- 新命令：`cnb-rs ...`
- 如果你想保留旧输入习惯，请自行配置 shell alias

完整迁移说明见：[从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)
