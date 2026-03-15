# cnb-rs release asset-delete

```
cnb-rs release asset-delete <TAG> --asset <NAME> [options]
```

删除 Release 中的指定附件。需要交互确认。

## 选项

- `<TAG>`: Tag 名称（必填）
- `--asset <NAME>`: 附件名称（必填）
- `--confirm`: 跳过交互确认

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除指定附件（需确认）
$ cnb-rs release asset-delete v1.2.0 --asset old-binary.tar.gz

# 跳过确认
$ cnb-rs release asset-delete v1.2.0 --asset old-binary.tar.gz --confirm
```

## API

| 方法   | 端点                                                |
| ------ | --------------------------------------------------- |
| DELETE | `/{repo}/-/releases/{release_id}/assets/{asset_id}` |

## 另请参阅

- [cnb-rs release](/release/)
- [cnb-rs release asset-clean](/release/asset-clean)
- [cnb-rs release view](/release/view)
