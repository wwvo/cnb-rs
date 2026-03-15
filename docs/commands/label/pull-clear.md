# cnb-rs label pull-clear

```
cnb-rs label pull-clear <number> [-y]
```

清空指定 Pull Request 的所有标签。

执行前会要求确认，可通过 `--yes` 跳过确认提示。

## 选项

- `<number>`: Pull 编号（必填）
- `-y, --yes`: 跳过确认提示

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 清空标签（需确认）
$ cnb-rs label pull-clear 10
确认清空 Pull #10 的所有标签？(y/N) y
✓ 已清空 Pull #10 的所有标签

# 跳过确认
$ cnb-rs label pull-clear 10 -y
```

## 另请参阅

- [cnb-rs label](/label/)
- [cnb-rs label pull-list](/label/pull-list)
- [cnb-rs label pull-set](/label/pull-set)
