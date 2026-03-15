# cnb-rs label pull-remove

```
cnb-rs label pull-remove <number> <name>
```

移除指定 Pull Request 的单个标签。

## 选项

- `<number>`: Pull 编号（必填）
- `<name>`: 标签名称（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs label pull-remove 10 "wip"
✓ 已从 Pull #10 移除标签: wip
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label pull-list](/label/pull-list)
- [cnb-rs label pull-clear](/label/pull-clear)
