# cnb-rs group create

```
cnb-rs group create <PATH> [options]
```

创建新组织。

## 选项

- `<PATH>`: 组织路径，作为唯一标识（必填）
- `-d, --description <TEXT>`: 组织描述
- `-r, --remark <TEXT>`: 备注
- `--bind-domain <DOMAIN>`: 根组织绑定的域名

**继承的全局选项：**

- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 创建基本组织
$ cnb-rs group create my-org

# 带描述和备注
$ cnb-rs group create my-org --description "我的组织" --remark "测试用"

# 绑定域名
$ cnb-rs group create my-org --bind-domain example.com
```

## 另请参阅

- [cnb-rs group](/group/)
- [cnb-rs group view](/group/view)
