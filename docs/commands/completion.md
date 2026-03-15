# cnb-rs completion

```
cnb-rs completion <shell>
```

生成指定 Shell 的命令行补全脚本并输出到 stdout。

支持的 Shell 类型：`bash`、`zsh`、`fish`、`powershell`、`elvish`。

## 示例

::: code-group

```bash [Bash]
$ cnb-rs completion bash >> ~/.bashrc
```

```bash [Zsh]
$ cnb-rs completion zsh >> ~/.zshrc
```

```bash [Fish]
$ cnb-rs completion fish > ~/.config/fish/completions/cnb-rs.fish
```

```powershell [PowerShell]
$ cnb-rs completion powershell >> $PROFILE
```

:::

## 另请参阅

- [cnb-rs](/guide/cnb)
