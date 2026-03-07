# cnb group settings

```
cnb group settings <GROUP> [options]
```

查看或更新组织配置。

不带修改选项时为查看模式，带修改选项时为更新模式。

## 选项

- `<GROUP>`: 组织路径（必填）
- `--hide-members <0|1>`: 是否对外隐藏组织成员（`0`=否, `1`=是）
- `--hide-sub-groups <0|1>`: 是否对外隐藏子组织（`0`=否, `1`=是）
- `--group-protection <0|1>`: 组织保护开关（`0`=关闭, `1`=开启）
- `--show-watermark <0|1>`: 是否显示私有仓库水印（`0`=否, `1`=是）

**继承的全局选项：**

- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 查看组织配置
$ cnb group settings my-org

# 隐藏成员列表
$ cnb group settings my-org --hide-members 1

# 开启组织保护
$ cnb group settings my-org --group-protection 1

# JSON 格式输出
$ cnb group settings my-org --json
```

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
