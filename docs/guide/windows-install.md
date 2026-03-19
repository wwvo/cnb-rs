# Windows 安装说明

## 推荐方式

当前 Windows 平台提供两类原生安装形态：

- `install.ps1`
  - 当前覆盖 `x86_64` 与 `arm64`
  - 自动识别系统架构，下载 `.zip` 压缩包到当前用户目录，并更新用户级 `PATH`
  - 不依赖管理员权限，也不需要导入 MSIX 证书
- `.msixbundle` / `.msix`
  - 当前覆盖 `x86_64-pc-windows-msvc` 与 `aarch64-pc-windows-msvc`
  - 使用 Windows 原生包安装机制，安装后通过 execution alias 调用 `cnb-rs`
- `.msi`
  - 当前覆盖 `x86_64-pc-windows-msvc`、`aarch64-pc-windows-msvc` 和 `x86_64-pc-windows-gnu`
  - 适合偏好传统机器级安装、系统级 `PATH` 和标准 MSI 卸载流程的用户

## 使用 `install.ps1` 一键安装

```powershell
irm https://raw.githubusercontent.com/wwvo/cnb-rs/main/install.ps1 | iex
```

如果你想固定某个版本，或者指定安装目录，建议先把脚本下载到本地再执行：

```powershell
Invoke-WebRequest https://raw.githubusercontent.com/wwvo/cnb-rs/main/install.ps1 -OutFile .\install.ps1
.\install.ps1 -Version v<VERSION> -InstallDir "$env:LOCALAPPDATA\Programs\cnb-rs\bin"
```

这个脚本会自动识别 Windows `x64 / arm64`，从 GitHub Release 下载对应 `.zip` 压缩包，校验 `sha256sum.txt`，然后把 `cnb-rs.exe` 安装到当前用户目录。它不会安装 `.msixbundle` / `.msi`，也不会处理 MSIX 证书导入。

## MSIX / MSIXBUNDLE

对大多数 MSVC 用户，推荐优先尝试以下附件：

- `cnb-rs-v<VERSION>-windows-msvc.msixbundle`
  - 同时包含 `x86_64-pc-windows-msvc` 与 `aarch64-pc-windows-msvc`
  - 适合希望让 Windows 自动选择体系结构的场景
- `cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msix`
  - 单独的 `x86_64` 安装包
- `cnb-rs-v<VERSION>-aarch64-pc-windows-msvc.msix`
  - 单独的 `arm64` 安装包

如果当前机器尚未信任该签名证书，建议先从同版本 Release 附件中下载 `cnb-rs-v<VERSION>-windows-msvc-signing-cert.cer`。如果你使用的是 CI / workflow 产物，也可以从同名 artifact 中获取这张证书。

首次侧载安装时可以先执行：

```powershell
Import-Certificate -FilePath .\cnb-rs-v<VERSION>-windows-msvc-signing-cert.cer -CertStoreLocation Cert:\CurrentUser\TrustedPeople
```

导入完成后，再安装 `.msixbundle` / `.msix`：

```powershell
Add-AppxPackage .\cnb-rs-v<VERSION>-windows-msvc.msixbundle
# 或按架构安装单独 .msix
Add-AppxPackage .\cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msix
Add-AppxPackage .\cnb-rs-v<VERSION>-aarch64-pc-windows-msvc.msix
```

如果 Windows 没有提示发布者未受信任，或者这张证书已经导入过，一般不需要在每次升级时重复导入。

MSIX 安装完成后，建议新开一个 PowerShell 或 CMD 窗口，再执行：

```powershell
cnb-rs --version
```

MSIX 路径行为说明：

- 通过 Windows execution alias 暴露 `cnb-rs`
- 不依赖 MSI 的 `C:\Program Files\cnb-rs` 目录约定
- 不会像 MSI 那样主动追加系统级 `PATH`

如果你的环境禁用了 `Add-AppxPackage`、策略上不允许侧载，或者你更偏好传统安装方式，可以继续使用下方的 MSI 安装。

## MSI

你也可以从 Release 页面下载对应目标的 `.msi` 后安装：

```powershell
msiexec /i .\cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msi
msiexec /i .\cnb-rs-v<VERSION>-aarch64-pc-windows-msvc.msi
msiexec /i .\cnb-rs-v<VERSION>-x86_64-pc-windows-gnu.msi
```

安装完成后，`cnb-rs` 默认会被安装到 `C:\Program Files\cnb-rs`，并自动追加到系统级 `PATH`。

你可以新开一个 PowerShell 或 CMD 窗口后执行：

```powershell
cnb-rs --version
```

## 静默安装

如果你希望在脚本中静默安装，可以使用：

```powershell
msiexec /i .\cnb-rs-v<VERSION>-<TARGET>.msi /qn /norestart
```

## 卸载

MSIX 可以在“已安装的应用”中卸载，也可以使用：

```powershell
Get-AppxPackage *cnb-rs* | Remove-AppxPackage
```

MSI 可以在“已安装的应用”中卸载，也可以使用静默卸载：

```powershell
msiexec /x .\cnb-rs-v<VERSION>-<TARGET>.msi /qn /norestart
```

卸载时会移除对应安装形态写入的程序文件；MSI 还会额外移除其写入的系统级 `PATH` 项。两种方式都不会主动删除用户配置或运行数据。

## 回退方式

如果你不希望使用 MSIX 或 MSI，或者当前目标不在上述覆盖范围内，仍然可以继续使用 Release 页面中的 `.zip` 压缩包附件：

1. 下载对应目标的 `.zip`
2. 解压到一个稳定目录，例如 `C:\Tools\cnb-rs`
3. 手工把该目录加入 `PATH`

## 当前边界

- 当前 `.msixbundle` / `.msix` 覆盖 `x86_64-pc-windows-msvc` 与 `aarch64-pc-windows-msvc`
- 当前 MSI 覆盖 `x86_64-pc-windows-msvc`、`aarch64-pc-windows-msvc` 和 `x86_64-pc-windows-gnu`
- 当前 `.msi` 与 `.msixbundle` / `.msix` 会同时保留，避免影响已有安装方式
- `aarch64-pc-windows-gnullvm` 当前仍以 `.zip` 为主
- 当前不自动修改 PowerShell `$PROFILE`
- 当前不内置 `cnb` -> `cnb-rs` 的 alias
- 当前不提供 winget 发布链路
