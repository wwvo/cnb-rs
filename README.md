# CNB CLI

CNB 平台专属命令行工具，在终端中高效管理你的 CNB 平台资源。

## 功能特性

- **认证管理** — 登录、查看状态、退出登录
- **AI 对话** — 使用自然语言与 CNB OpenAPI 交互
- **Issue 管理** — 创建、列出、关闭、评论、下载 Issue
- **Pull Request 管理** — 创建、更新、合并 PR
- **Release 管理** — 创建 Release、上传/清理附件
- **Commit 管理** — 上传/清理 Commit 附件
- **文件下载** — 按路径、分支下载仓库文件，支持 glob 过滤
- **统计仪表盘** — 提交排行榜、提交趋势图、Star 趋势图（TUI）
- **知识库管理** — 查询、清除仓库 Embedding 知识库
- **组织管理** — 更新组织 Logo
- **工作区管理** — 清理已关闭的云原生工作区
- **命令行补全** — 支持 Bash、Zsh、Fish、PowerShell

## 安装

从 [Release 页面](https://cnb.cool/prevailna/cnb/-/releases) 下载适合你平台的二进制文件。

## 快速开始

```bash
# 登录
cnb auth login

# 查看仓库信息
cnb info

# 查看 Issue 列表
cnb issue list

# 使用 AI 对话
cnb chat --do "查看我的仓库列表"

# 下载文件
cnb download --files README.md
```

CNB CLI 也可以通过 `git cnb` 方式调用：

```bash
git cnb info
git cnb issue list
```

## 文档

完整文档请访问：

- **https://cnb.wwvo.fun**
- **https://cnba.pages.dev**

## 许可证

[Apache-2.0](LICENSE)
