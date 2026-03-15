# cnb-rs release download

```
cnb-rs release download <TAG> <FILENAME> [options]
```

下载 Release 附件到本地，或生成分享下载链接。

## 选项

- `<TAG>`: Tag 名称（必填）
- `<FILENAME>`: 要下载的文件名（必填）
- `-o, --output <PATH>`: 保存路径（可以是目录或完整文件路径）
- `--share`: 仅生成分享下载链接，不下载文件

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 下载附件到当前目录
$ cnb-rs release download v1.2.0 app-linux-amd64.tar.gz

# 保存到指定目录
$ cnb-rs release download v1.2.0 app-linux-amd64.tar.gz -o ./downloads/

# 保存为指定文件名
$ cnb-rs release download v1.2.0 app-linux-amd64.tar.gz -o ./app.tar.gz

# 获取分享下载链接
$ cnb-rs release download v1.2.0 app-linux-amd64.tar.gz --share
```

## API

| 方法 | 端点                                           |
| ---- | ---------------------------------------------- |
| GET  | `/{repo}/-/releases/download/{tag}/{filename}` |

## 另请参阅

- [cnb-rs release](/release/)
- [cnb-rs release view](/release/view)
- [cnb-rs release asset-upload](/release/asset-upload)
