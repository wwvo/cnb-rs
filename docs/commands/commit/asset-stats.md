# cnb-rs commit asset-stats

```
cnb-rs commit asset-stats
```

统计仓库所有 Commit 的附件总大小。

遍历仓库所有 Commit，累加每个 Commit 下所有附件的大小，输出总大小（原始字节数和可读格式）。

## 选项

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 输出示例

```
Total Size: 15728640 (15.0 MB)
```

## 另请参阅

- [cnb-rs commit](/commit/)
