---
title: cnb-rs completion
---

# cnb-rs completion

::: details 包管理器安装

如果你是通过包管理器安装 `cnb-rs`，有些发行方式可能已经随安装过程放置好了补全脚本。

如果补全已经可以直接使用，就不需要再手动执行下面的配置步骤。

如果补全没有自动生效，或者你希望手动注册补全，请继续参考下文。

对于通过 Homebrew 安装的场景，Formula 会生成补全脚本，但 shell 是否立即可用仍取决于 Homebrew completion 支持是否已启用。可以先参考 Homebrew 官方文档：

- <https://docs.brew.sh/Shell-Completion>

:::

## 用法

```
cnb-rs completion -s <shell>
cnb-rs completion --shell <shell>
```

生成指定 Shell 的命令行补全脚本。

- 支持的 Shell 类型：`bash`、`zsh`、`fish`、`powershell`、`elvish`。
- 不再支持位置参数写法，例如 `cnb-rs completion bash`；请改用 `-s` 或 `--shell`。
- 运行 `cnb-rs completion --help` 时，CLI 会输出精简帮助，并指向这份文档：<https://cnb.wwvo.fun/completion>。
- `completion` 子命令只接受 `--shell` 和 `--help`。像 `--domain`、`--repo`、`--json` 这样的全局参数不适用于这里，也不会继续出现在对应的补全候选里。

> [!WARNING]
> 改名后补全文件名也随之变化。例如 Fish 的输出文件应从 `cnb.fish` 改为 `cnb-rs.fish`。升级旧版本后，请重新生成补全脚本或重新加载 profile，避免继续使用旧的补全定义。

## Bash

先确保你已经通过系统包管理器安装了 `bash-completion`。

然后把下面这一行加入 `~/.bash_profile`：

```bash
eval "$(cnb-rs completion -s bash)"
```

如果你的环境主要从 `~/.bashrc` 加载配置，也可以加到 `~/.bashrc` 中。

保存后重新打开一个 Bash 窗口；如果你想立刻在当前会话生效，也可以执行：

```bash
source ~/.bash_profile
```

## Zsh

生成一个 `_cnb-rs` 补全脚本，并把它放到 `$fpath` 中的某个目录，例如：

```zsh
cnb-rs completion -s zsh > /usr/local/share/zsh/site-functions/_cnb-rs
```

请确保你的 `~/.zshrc` 中包含：

```zsh
autoload -U compinit
compinit -i
```

推荐使用 Zsh 5.7 或更高版本。

如果你不想写入系统目录，也可以改用用户目录，例如先创建 `~/.zsh/completions`，把 `_cnb-rs` 放进去，并确保该目录已经加入 `$fpath`。

保存后重新打开一个 Zsh 窗口；如果你想立刻在当前会话生效，也可以重新执行：

```zsh
autoload -U compinit
compinit -i
```

## Fish

生成一个 `cnb-rs.fish` 补全脚本：

```fish
mkdir -p ~/.config/fish/completions
cnb-rs completion -s fish > ~/.config/fish/completions/cnb-rs.fish
```

保存后重新打开一个 Fish 窗口即可。

## PowerShell

先打开你的 PowerShell profile：

```powershell
mkdir -Path (Split-Path -Parent $PROFILE) -ErrorAction SilentlyContinue
notepad $PROFILE
```

如果你平时直接使用 `cnb-rs`，把下面这一行加入 `$PROFILE` 后保存：

```powershell
Invoke-Expression -Command $(cnb-rs completion -s powershell | Out-String)
```

如果你还配置了 `Set-Alias cnb cnb-rs`，建议把下面两行加入 `$PROFILE`。这样 `cnb-rs` 和 `cnb` 都能触发 Tab 补全：

```powershell
Set-Alias cnb cnb-rs
Invoke-Expression -Command $((cnb-rs completion -s powershell | Out-String) -replace [regex]::Escape("-CommandName 'cnb-rs'"), "-CommandName 'cnb-rs', 'cnb'")
```

如果你是从旧版本升级过来的，建议重新执行一次 `$PROFILE` 里的这段注册逻辑。`Register-ArgumentCompleter` 只在当前会话生效，旧会话不会自动切换到新的补全定义。

保存后，重新打开一个 PowerShell 窗口；如果你想立刻在当前会话生效，也可以执行：

```powershell
. $PROFILE
```

## Elvish

把补全脚本追加到 Elvish 的 `rc.elv`。如果你没有显式设置 `XDG_CONFIG_HOME`，默认路径通常是：

- Linux / macOS：`~/.config/elvish/rc.elv`
- Windows：`%AppData%\\elvish\\rc.elv`

例如在 Linux / macOS 上可以执行：

```bash
mkdir -p ~/.config/elvish
cnb-rs completion -s elvish >> ~/.config/elvish/rc.elv
```

保存后重新打开一个 Elvish 窗口即可生效。

## 另请参阅

- [cnb-rs](/cnb)
- [从 cnb 迁移到 cnb-rs](/migrate-cnb-to-cnb-rs)
