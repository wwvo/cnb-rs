# cnb release asset-upload

```
cnb release asset-upload [flags]
```

上传本地文件到指定 Release 作为附件。

通过 Tag 名称定位 Release，获取上传 URL 后将文件上传至对象存储，最后确认上传完成。

## 选项

`-t, --tag-name <TAG>`
: Release 的 Tag 名称（必填）

`-f, --file <FILE>`
: 要上传的本地文件路径（必填）

## 示例

```bash
# 上传文件到指定 Release
$ cnb release asset-upload --tag-name v1.0.0 --file dist/app.zip
文件 app.zip 已上传到 Release v1.0.0

# 简写
$ cnb release asset-upload -t v1.0.0 -f ./build/output.tar.gz
```

## 另请参阅

- [cnb release](/release/)
