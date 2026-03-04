# cnb knowledge query

```
cnb knowledge query [flags]
```

查询当前仓库的知识库，返回与查询文本语义相关的结果。

每条结果输出格式：`URL;分数;文件类型`

## 选项

`-q, --query <QUERY>`
: 查询文本（必填）

`--score-threshold <THRESHOLD>`
: 分数阈值，仅返回分数高于此值的结果

`--top-k <N>`
: 返回结果数量上限

## 示例

```bash
# 基本查询
$ cnb knowledge query --query "如何配置认证"

# 指定分数阈值和返回数量
$ cnb knowledge query -q "登录流程" --score-threshold 0.7 --top-k 5
```

## 输出示例

```
src/auth/login.rs;0.9234;code
docs/guide/auth.md;0.8765;markdown
```

## 另请参阅

- [cnb knowledge](/knowledge/)
