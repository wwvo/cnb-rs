# cnb-rs release

Release 版本管理，包括查看、创建、更新、删除 Release，以及附件的上传、下载、统计和清理等操作。

## 可用命令

- [cnb-rs release list](/release/list) — 列出 Release
- [cnb-rs release view](/release/view) — 查看 Release 详情
- [cnb-rs release create](/release/create) — 创建 Release
- [cnb-rs release update](/release/update) — 更新 Release
- [cnb-rs release delete](/release/delete) — 删除 Release
- [cnb-rs release latest](/release/latest) — 查看最新 Release
- [cnb-rs release download](/release/download) — 下载 Release 附件
- [cnb-rs release asset-upload](/release/asset-upload) — 上传附件到 Release
- [cnb-rs release asset-delete](/release/asset-delete) — 删除单个附件
- [cnb-rs release asset-stats](/release/asset-stats) — 统计 Release 附件大小
- [cnb-rs release asset-clean](/release/asset-clean) — 清理 Release 附件

## 示例

```bash
# 列出 Release
$ cnb-rs release list

# 查看最新 Release
$ cnb-rs release latest

# 创建并上传附件
$ cnb-rs release create -t v1.0.0 -n "v1.0.0"
$ cnb-rs release asset-upload -t v1.0.0 -f ./dist/app.tar.gz

# 下载附件
$ cnb-rs release download v1.0.0 app.tar.gz
```

## 另请参阅

- [cnb-rs](/guide/cnb)
