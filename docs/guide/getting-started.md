# 快速开始

## 安装

从 [Release 页面](https://cnb.cool/wwvo/cnb-rs/cnb-rs/-/releases) 下载适合你平台的发布产物。

### Linux

当前 Linux 原生包覆盖 `x86_64-unknown-linux-gnu` 和 `aarch64-unknown-linux-gnu`：

```bash
# Debian / Ubuntu（x86_64）
sudo dpkg -i ./cnb-rs_*_amd64.deb

# Debian / Ubuntu（arm64）
sudo dpkg -i ./cnb-rs_*_arm64.deb

# Fedora / Rocky / AlmaLinux（x86_64）
sudo dnf install ./cnb-rs-*.x86_64.rpm

# Fedora / Rocky / AlmaLinux（arm64）
sudo dnf install ./cnb-rs-*.aarch64.rpm
```

如果你的环境不适合 `.deb` / `.rpm`，也可以继续下载 `.tar.gz` 手动解压，并把 `cnb-rs` 放到 `PATH` 中。

> [!NOTE]
> 当前只提供 release 附件下载，不提供 apt / yum 软件源托管；其他 Linux 目标目前仍以 `.tar.gz` 为主。

更多说明见：[Linux 安装说明](/guide/linux-install)。

### Windows

Windows `x86_64` 当前优先提供 `.msi` 安装包，推荐直接双击安装，或在 PowerShell 中静默安装：

```powershell
msiexec /i .\cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msi
```

如果你不希望使用 MSI，或者当前目标不是 `x86_64-pc-windows-msvc`，也仍然可以继续使用 release 页面中的 `.zip` 压缩包附件。

> [!NOTE]
> 当前 MSI 只覆盖 `x86_64-pc-windows-msvc`。Windows 其他目标仍以 `.zip` 附件为主。

更多说明见：[Windows 安装说明](/guide/windows-install)。

### macOS

macOS 当前继续使用 release 页面中的 `.tar.gz` 压缩包附件。

如果你更偏好包管理器方式，也可以直接使用：

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

边界说明：

- `Homebrew` 当前适合 macOS，以及偏好使用 brew 的 Linux 用户
- `Scoop` 当前适合 Windows 用户
- 这两种方式依赖外部分发仓库，不替代 release 页面附件或 MSI 安装包

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
- Linux 用户若改用 `.deb` / `.rpm` 安装，补全文件会随包一起安装，无需再手工拷贝 Bash / Zsh / Fish 补全脚本

完整迁移说明见：[从 cnb 迁移到 cnb-rs](/guide/migrate-cnb-to-cnb-rs)
