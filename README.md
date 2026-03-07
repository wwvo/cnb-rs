<div align="center">

# CNB CLI

**一个非官方的 [CNB](https://cnb.cool) 平台命令行工具**

在终端中高效管理你的 CNB 平台资源。

[![License](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-edition%202024-orange?style=flat-square)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)]()
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=flat-square)]()

[功能特性](#功能特性) · [安装](#安装) · [快速开始](#快速开始) · [文档](https://cnb.wwvo.fun) · [项目结构](#项目结构)

</div>

---

> [!NOTE]
> 官方 CLI 工具请访问 [looc/git-cnb](https://cnb.cool/looc/git-cnb)。

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

从 [Release 页面](https://cnb.cool/wwvo/cnb-cli/cnb/-/releases) 下载适合你平台的二进制文件。

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

也可以通过 `git cnb` 方式调用：

```bash
git cnb info
git cnb issue list
```

## 项目结构

```
cnb/
├── src/                  # 主程序入口 (cnb / git-cnb)
│   ├── main.rs
│   └── commands/         # 命令实现
├── crates/
│   ├── cnb-api/          # CNB 平台 API 客户端
│   ├── cnb-chat/         # AI 对话功能
│   ├── cnb-core/         # 核心类型与上下文
│   └── cnb-tui/          # 终端 UI（统计、趋势图）
└── docs/                 # VitePress 文档站点
```

## 从源码构建

**前置条件：** [Rust](https://www.rust-lang.org/tools/install)（edition 2024）

```bash
git clone https://cnb.cool/wwvo/cnb-cli/cnb.git
cd cnb
cargo build --release
```

产物路径：`target/release/cnb`（Windows 下为 `target/release/cnb.exe`）。

## 文档

- **https://cnb.wwvo.fun**
- **https://cnba.pages.dev**

## 许可证

[Apache-2.0](LICENSE)
