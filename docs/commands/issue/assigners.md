# cnb-rs issue assigners

```
cnb-rs issue assigners <subcommand>
```

Issue 处理人管理，支持获取和添加处理人。

## 子命令

### assigners get

```
cnb-rs issue assigners get <NUMBER>
```

获取指定 Issue 的处理人列表。

- `<NUMBER>`: Issue 编号（必填）

### assigners add

```
cnb-rs issue assigners add <NUMBER> [flags]
```

为指定 Issue 添加处理人。多个用户名用逗号分隔，自动去重。

- `<NUMBER>`: Issue 编号（必填）
- `-a, --assignees <ASSIGNEES>`: 处理人用户名，多个用逗号分隔（必填）

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出（适用于 get）
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 示例

```bash
# 获取处理人列表
$ cnb-rs issue assigners get 123
USERNAME        NICKNAME
zhangsan        张三
lisi            李四

# JSON 格式
$ cnb-rs --json issue assigners get 123

# 添加单个处理人
$ cnb-rs issue assigners add 123 -a zhangsan
✓ Issue #123 处理人已更新

# 添加多个处理人
$ cnb-rs issue assigners add 123 -a "zhangsan,lisi"
✓ Issue #123 处理人已更新
```

## 另请参阅

- [cnb-rs issue](/issue/)
- [cnb-rs issue edit](/issue/edit)
