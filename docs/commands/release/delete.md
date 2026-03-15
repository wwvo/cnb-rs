# cnb-rs release delete

```
cnb-rs release delete <TAG> [options]
```

删除指定 Release。此操作不可逆，需要交互确认。

## 选项

- `<TAG>`: Tag 名称（必填）
- `--confirm`: 跳过交互确认

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 删除 Release（需确认）
$ cnb-rs release delete v1.0.0-beta

# 跳过确认直接删除
$ cnb-rs release delete v1.0.0-beta --confirm
```

## API

| 方法   | 端点                              |
| ------ | --------------------------------- |
| DELETE | `/{repo}/-/releases/{release_id}` |

## 另请参阅

- [cnb-rs release](/release/)
- [cnb-rs release list](/release/list)
