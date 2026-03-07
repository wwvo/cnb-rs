# cnb badge upload

```
cnb badge upload --key <key> --sha <sha> --message <msg> [options]
```

上传自定义徽章数据。

自定义徽章用于在 CI/CD 流水线中上报代码质量、安全扫描等自定义指标。
上传后可通过 `cnb badge get` 获取对应的 SVG 或 JSON 数据。

## 选项

- `-k, --key <KEY>`: 徽章 key，如 `security/tca`（必填）
- `--sha <SHA>`: Commit ID（必填）
- `-m, --message <MSG>`: 徽章右侧显示内容（必填）
- `--value <NUM>`: 徽章数值
- `-l, --link <URL>`: 点击徽章右侧的跳转链接
- `--latest`: 同时上传 latest 徽章

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb badge upload --key security/tca --sha abc12345 \
    --message "A+" --value 95 \
    --link "https://example.com/report" \
    --latest
✓ 徽章已上传
  Commit URL: https://cnb.cool/.../badge/git/abc12345/security/tca
  Latest URL: https://cnb.cool/.../badge/git/latest/security/tca
```

## 另请参阅

- [cnb badge](/badge/)
- [cnb badge get](/badge/get)
- [cnb badge list](/badge/list)
