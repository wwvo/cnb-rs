# cnb-rs group update-logo

```
cnb-rs group update-logo [flags]
```

更新指定组织的 Logo。

上传本地图片文件作为组织 Logo，文件通过对象存储上传。

## 选项

- `-g, --group <GROUP>`: 组织名称（必填）
- `--logo-path <FILE>`: Logo 文件路径（必填）

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
$ cnb-rs group update-logo --group my-org --logo-path ./logo.png
组织 Logo 更新成功
```

## 另请参阅

- [cnb-rs group](/group/)
- [cnb-rs group update](/group/update)
