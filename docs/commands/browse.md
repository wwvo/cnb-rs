# cnb-rs browse

```
cnb-rs browse [path]
```

在浏览器中打开当前仓库的页面。

根据当前 Git 仓库的 remote 地址自动推断仓库 URL，并在默认浏览器中打开。
可通过 `[path]` 参数指定打开的子页面。

## 选项

- `[path]`: 子路径，如 `-/issues`、`-/pulls`、`-/settings`

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 打开仓库首页
$ cnb-rs browse

# 打开 Issues 页面
$ cnb-rs browse -/issues

# 打开 Pull Requests 页面
$ cnb-rs browse -/pulls

# 打开仓库设置页面
$ cnb-rs browse -/settings
```

## 另请参阅

- [cnb-rs](/guide/cnb)
- [cnb-rs info](/info)
