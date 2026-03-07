# 快速开始

## 安装

从 [Release 页面](https://cnb.cool/wwvo/cnb-cli/cnb/-/releases) 下载适合你平台的二进制文件。

## 登录

```bash
cnb auth login
# 或直接指定 Token
cnb auth login --token <YOUR_TOKEN>
```

## 基本使用

```bash
# 查看当前认证状态
cnb auth status

# 查看仓库信息
cnb info

# 查看 Issue 列表
cnb issue list

# 使用 AI 对话
cnb chat --do "查看我的 Issue 列表"
```

## 作为 Git 子命令

CNB CLI 也可以通过 `git cnb` 方式调用：

```bash
git cnb info
git cnb issue list
```
