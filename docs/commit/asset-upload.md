# cnb commit asset-upload

```
cnb commit asset-upload [flags]
```

上传本地文件到指定 Commit 作为附件。

通过 Commit SHA1 定位，获取上传 URL 后将文件上传至对象存储，最后确认上传完成。

## 选项

`-s, --sha1 <SHA1>`
: Commit 的 SHA1 值（必填）

`-f, --file <FILE>`
: 要上传的本地文件路径（必填）

## 示例

```bash
# 上传文件到指定 Commit
$ cnb commit asset-upload --sha1 abc123def456 --file dist/app.zip
文件 app.zip 已上传到 Commit abc123def456

# 简写
$ cnb commit asset-upload -s abc123def456 -f ./build/output.tar.gz
```

## 另请参阅

- [cnb commit](/commit/)
