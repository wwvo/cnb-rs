# cnb group quota

```
cnb group quota <GROUP>
```

查看组织的特权额度信息，包括算力、存储等资源配额。

## 参数

`GROUP`
: 组织路径（必填）

## 输出

| 资源类型 | 说明 |
|------|------|
| 构建算力 | CI/CD 构建使用的 CPU 核时 |
| 构建 GPU 算力 | CI/CD 构建使用的 GPU 核时 |
| 开发算力 | 云原生开发环境使用的 CPU 核时 |
| 开发 GPU 算力 | 云原生开发环境使用的 GPU 核时 |
| Git 存储 | Git 仓库存储容量 (GiB) |
| 对象存储 | 对象存储容量 (GiB) |

## 示例

```bash
# 查看额度
$ cnb group quota my-org

# JSON 输出
$ cnb group quota my-org --json
```

## API

| 方法 | 端点 |
|------|------|
| GET | `/{slug}/-/charge/special-amount` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
