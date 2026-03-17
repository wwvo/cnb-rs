# Windows 安装说明

## 推荐方式

Windows `x86_64` 用户当前优先使用 Release 页面中的 `.msi` 安装包：

```powershell
msiexec /i .\cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msi
```

安装完成后，`cnb-rs` 默认会被安装到 `C:\Program Files\cnb-rs`，并自动追加到系统级 `PATH`。

你可以新开一个 PowerShell 或 CMD 窗口后执行：

```powershell
cnb-rs --version
```

## 静默安装

如果你希望在脚本中静默安装，可以使用：

```powershell
msiexec /i .\cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msi /qn /norestart
```

## 卸载

可以在“已安装的应用”中卸载，也可以使用静默卸载：

```powershell
msiexec /x .\cnb-rs-v<VERSION>-x86_64-pc-windows-msvc.msi /qn /norestart
```

卸载时会移除 MSI 安装的文件，以及 MSI 写入的系统级 `PATH` 项；不会主动删除用户配置或运行数据。

## 回退方式

如果你不希望使用 MSI，或者当前目标不是 `x86_64-pc-windows-msvc`，仍然可以继续使用 Release 页面中的 `.zip` 压缩包附件：

1. 下载对应目标的 `.zip`
2. 解压到一个稳定目录，例如 `C:\Tools\cnb-rs`
3. 手工把该目录加入 `PATH`

## 当前边界

- 当前只有 `x86_64-pc-windows-msvc` 提供 `.msi`
- Windows 其他目标当前仍以 `.zip` 为主
- 当前不自动修改 PowerShell `$PROFILE`
- 当前不内置 `cnb` -> `cnb-rs` 的 alias
- 当前不提供 winget 发布链路
