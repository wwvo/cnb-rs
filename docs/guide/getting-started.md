# 快速开始

## 安装

从 [Release 页面](https://cnb.cool/wwvo/cnb-cli/cnb/-/releases) 下载适合你平台的二进制文件。

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
