# cnb release create

```
cnb release create [flags]
```

创建 Release。

创建成功后输出 Release 的 Web 链接。`target_commitish` 默认使用 Tag 名称。

## 选项

`-t, --tag <TAG>`
: Release 的 Tag 名称（必填）

`-n, --name <NAME>`
: Release 名称（必填）

`-b, --body <BODY>`
: Release 描述（默认: 空）

`--make-latest <true|false>`
: 是否标记为最新版本（默认: `false`）

`--prerelease`
: 标记为预发布版本

## 示例

```bash
# 创建 Release
$ cnb release create --tag v1.0.0 --name v1.0.0

# 创建预发布版本
$ cnb release create -t v2.0.0-beta -n "v2.0.0 Beta" --prerelease

# 标记为最新版本并添加描述
$ cnb release create -t v1.0.0 -n v1.0.0 --make-latest true -b "首个正式版本"
```

## 另请参阅

- [cnb release](/release/)
