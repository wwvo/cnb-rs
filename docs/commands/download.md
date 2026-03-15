# cnb-rs download

```
cnb-rs download [flags]
```

从仓库下载原始文件，支持 glob 模式匹配和并发下载。

## 选项

- `--files <FILES>`: 要下载的文件路径（逗号分隔或多次指定）
- `--ref <REF>`: Git 引用（分支/tag/commit），默认使用默认分支
- `--local-dir <DIR>`: 本地下载目录（默认：`.`）
- `--include <PATTERN>`: 包含的文件 glob 模式（可多次指定）
- `--exclude <PATTERN>`: 排除的文件 glob 模式（可多次指定）
- `-c, --concurrency <N>`: 最大并发下载数（默认：`4`）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 下载指定文件
$ cnb-rs download --files README.md,LICENSE

# 下载到指定目录
$ cnb-rs download --files src/ --local-dir ./output

# 使用 glob 模式过滤
$ cnb-rs download --include "*.rs" --exclude "test_*"

# 从指定分支下载
$ cnb-rs download --files docs/ --ref develop
```

## 另请参阅

- [cnb-rs](/guide/cnb)
