# 从 cnb 迁移到 cnb-rs

`cnb` 已在本项目中正式改名为 `cnb-rs`。

这次改名的目标是和官方 CLI 明确区分。当前用户本地执行的命令名、发布产物名、CNB 仓库路径和 GitHub 仓库名都已经对齐到 `cnb-rs`。

## 你需要改什么

- 命令调用：`cnb ...` 改为 `cnb-rs ...`
- 可执行文件名：`cnb` / `cnb.exe` 改为 `cnb-rs` / `cnb-rs.exe`
- 发布产物名：`cnb-<tag>-<target>` 改为 `cnb-rs-<tag>-<target>`
- Fish 补全文件：`cnb.fish` 改为 `cnb-rs.fish`

## 常见命令映射

```text
cnb auth login            -> cnb-rs auth login
cnb auth status           -> cnb-rs auth status
cnb info                  -> cnb-rs info
cnb issue list            -> cnb-rs issue list
cnb pull create           -> cnb-rs pull create
cnb release latest        -> cnb-rs release latest
cnb completion fish       -> cnb-rs completion fish
```

## 如果你想继续输入 cnb

本项目不在程序内恢复 `cnb` 入口，也不在主仓库中内置 wrapper。更推荐由用户本地自行配置 alias，或由外部分发仓库在安装说明里提供迁移 note。

### Bash / Zsh

```bash
alias cnb='cnb-rs'
```

### Fish

```fish
alias cnb cnb-rs
```

### PowerShell

```powershell
Set-Alias cnb cnb-rs
```

如果你选择 alias，请注意：

- 文档与 release notes 仍然会统一使用 `cnb-rs`
- 问题排查时，也优先按 `cnb-rs` 的实际可执行文件名检查 PATH 和安装状态

## 当前仓库标识

当前仓库入口已经更新为：

- 仓库地址：`https://cnb.cool/wwvo/cnb-rs/cnb-rs.git`
- 仓库 slug：`wwvo/cnb-rs/cnb-rs`

## 仍保持兼容的部分

以下内容不因命令改名而改变：

- 配置目录：`~/.cnb/`
- 现有 token 环境变量名，例如 `CNB_TOKEN`

## 发布与升级建议

升级到改名后的版本时，建议按下面顺序检查：

1. 下载新的 `cnb-rs-<tag>-<target>` 产物
2. 确认 PATH 中实际可执行文件名为 `cnb-rs`
3. 如果使用补全，重新生成或替换对应的补全文件
4. 如果需要保留旧习惯，再额外配置 alias

## 另请参阅

- [快速开始](/guide/getting-started)
- [cnb-rs 命令总览](/guide/cnb)
- [发布流程](/guide/release-process)
