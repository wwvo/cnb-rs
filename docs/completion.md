# cnb completion

```
cnb completion <shell>
```

生成指定 Shell 的命令行补全脚本并输出到 stdout。

支持的 Shell 类型：`bash`、`zsh`、`fish`、`powershell`、`elvish`。

## 示例

::: code-group

```bash [Bash]
$ cnb completion bash >> ~/.bashrc
```

```bash [Zsh]
$ cnb completion zsh >> ~/.zshrc
```

```bash [Fish]
$ cnb completion fish > ~/.config/fish/completions/cnb.fish
```

```powershell [PowerShell]
$ cnb completion powershell >> $PROFILE
```

:::

## 另请参阅

- [cnb](/guide/cnb)
