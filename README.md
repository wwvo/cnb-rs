<div align="center">

# cnb-rs

**一个非官方的 [CNB](https://cnb.cool) 平台命令行工具**

在终端中高效管理你的 CNB 平台资源。

[![CNB Repo](https://img.shields.io/badge/CNB-wwvo%2Fcnb--rs-2F80ED?style=flat-square&logo=cloudnativebuild&logoColor=white)](https://cnb.cool/wwvo/cnb-rs/cnb-rs)
[![GitHub Repo](https://img.shields.io/badge/GitHub-wwvo%2Fcnb--rs-181717?style=flat-square&logo=github&logoColor=white)](https://github.com/wwvo/cnb-rs)
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

从 [Release 页面](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/releases) 下载适合你平台的发布产物。

如果你想直接一键安装，也可以使用仓库里的安装脚本：

```bash
curl -fsSL https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/git/raw/main/scripts/install.sh | bash
```

```powershell
irm https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/git/raw/main/scripts/install.ps1 | iex
```

- `main/scripts/install.sh` 和 `main/scripts/install.ps1` 适合“始终获取最新安装脚本”的场景
- 每个 Release 也会附带版本化脚本 `cnb-rs-v<VERSION>-install.sh` 与 `cnb-rs-v<VERSION>-install.ps1`，适合固定版本安装或离线保存后执行

- 如果你希望安装脚本改从 GitHub Release 下载，也可以切换下载源：

```bash
CNB_RS_INSTALL_SOURCE=github curl -fsSL https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/git/raw/main/scripts/install.sh | bash
```

```powershell
$env:CNB_RS_INSTALL_SOURCE = "github"; irm https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/git/raw/main/scripts/install.ps1 | iex
```

- `scripts/install.sh` 会自动识别 Linux / macOS 的系统和架构，默认安装到可写的 `/usr/local/bin`，否则回退到 `~/.local/bin`
- `scripts/install.ps1` 会自动识别 Windows `x64 / arm64`，默认安装到当前用户的 `AppData\Local\Programs\cnb-rs\bin`，并尝试追加到用户级 `PATH`
- 两个脚本默认都会从 CNB Release 下载压缩包，也可以通过 `CNB_RS_INSTALL_SOURCE=github`、`--source github` 或 `-Source github` 切换到 GitHub Release，并继续校验 `sha256sum.txt`
- 如果你下载的是某个 Release 自带的 `cnb-rs-v<VERSION>-install.sh` 或 `cnb-rs-v<VERSION>-install.ps1`，它们默认会指向同版本的发布产物
- 如果你不显式指定版本，脚本会使用仓库当前维护的默认发布版本
- 这两个脚本只安装 `cnb-rs` 二进制文件，不替代 `.deb` / `.rpm` / `.msix` / `.msi` 的完整安装形态

- Linux `x86_64 / arm64`：优先使用 `.deb` 或 `.rpm`
- Windows `x86_64 / arm64`（MSVC）：可选已签名 `.msixbundle`；如需传统机器级安装，也可继续使用对应 `.msi`
- Windows `x86_64`（GNU）：使用 `x86_64-pc-windows-gnu.msi`
- Windows 其他目标：当前继续使用 `.zip` 压缩包附件
- macOS：当前继续使用 `.tar.gz` 压缩包附件
- 其他 Linux 目标，或你的发行版不适用 `.deb` / `.rpm` 时，仍然可以回退到 `.tar.gz` 手动解压安装

Linux 安装细节见：[Linux 安装说明](docs/guide/linux-install.md)。
Windows 安装细节见：[Windows 安装说明](docs/guide/windows-install.md)。

如果你更喜欢包管理器安装，也可以使用：

```bash
# Homebrew（macOS / Linux）
brew tap wwvo/cnb-rs https://cnb.cool/wwvo/cnb-rs/homebrew-cnb-rs.git
brew install wwvo/cnb-rs/cnb-rs
```

```powershell
# Scoop（Windows）
scoop bucket add cnb-rs https://cnb.cool/wwvo/cnb-rs/scoop-cnb-rs.git
scoop install cnb-rs/cnb-rs
```

> [!WARNING]
> 从本项目的改名版本开始，原来的 `cnb ...` 已改为 `cnb-rs ...`。
> 如果你是从旧版本升级，请先阅读 [迁移指南](docs/guide/migrate-cnb-to-cnb-rs.md)。

## 快速开始

```bash
# 登录
cnb-rs auth login

# 查看仓库信息
cnb-rs info

# 查看 Issue 列表
cnb-rs issue list

# 使用 AI 对话
cnb-rs chat --do "查看我的仓库列表"

# 下载文件
cnb-rs download --files README.md
```

如果你通过 Linux 原生包安装，`cnb-rs` 会同时安装 Bash、Zsh、Fish 补全文件；PowerShell 补全仍建议按 [completion 文档](docs/commands/completion.md) 手动生成。

## 项目结构

```
cnb/
├── src/                  # 主程序入口
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
git clone https://cnb.cool/wwvo/cnb-rs/cnb-rs.git
cd cnb-rs
cargo build --release
```

产物路径：`target/release/cnb-rs`（Windows 下为 `target/release/cnb-rs.exe`）。

## 迁移说明

- 旧命令：`cnb ...`
- 新命令：`cnb-rs ...`
- 如果需要继续输入 `cnb`，建议在本地 shell 中自行配置 alias，而不是依赖程序内兼容层

完整说明见：[从 cnb 迁移到 cnb-rs](docs/guide/migrate-cnb-to-cnb-rs.md)

## 文档

- **https://cnb.wwvo.fun**
- **https://cnba.pages.dev**

## 许可证

[Apache-2.0](LICENSE)
