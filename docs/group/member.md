# cnb group member

组织成员管理。包含 list、add、update、remove 四个子命令。

## 子命令

### member list

```
cnb group member list <GROUP> [options]
```

列出组织的直接成员。

**参数：**

`GROUP`
: 组织路径（必填）

**选项：**

`-r, --role <ROLE>`
: 按角色过滤

`-s, --search <KEYWORD>`
: 关键字搜索

**输出列：**

| 列 | 说明 |
|------|------|
| 用户名 | 成员用户名 |
| 昵称 | 成员昵称 |
| 权限 | 权限级别 |
| 加入时间 | 加入组织的时间 |

---

### member add

```
cnb group member add <GROUP> <USERNAME> [options]
```

添加组织成员。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 要添加的用户名（必填）

**选项：**

`-l, --level <LEVEL>`
: 权限级别（默认：Developer）。可选值：Guest/Reporter/Developer/Master/Owner

---

### member update

```
cnb group member update <GROUP> <USERNAME> --level <LEVEL>
```

更新成员权限。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 用户名（必填）

**选项：**

`-l, --level <LEVEL>`
: 新的权限级别（必填）。可选值：Guest/Reporter/Developer/Master/Owner

---

### member remove

```
cnb group member remove <GROUP> <USERNAME> [options]
```

移除组织成员。需要交互确认。

**参数：**

`GROUP`
: 组织路径（必填）

`USERNAME`
: 要移除的用户名（必填）

**选项：**

`--confirm`
: 跳过交互确认

## 示例

```bash
# 列出成员
$ cnb group member list my-org

# 按角色过滤
$ cnb group member list my-org --role Owner

# 添加成员
$ cnb group member add my-org alice --level Developer

# 更新权限
$ cnb group member update my-org alice --level Master

# 移除成员
$ cnb group member remove my-org alice

# JSON 输出
$ cnb group member list my-org --json
```

## API

| 操作 | 方法 | 端点 |
|------|------|------|
| 列出成员 | GET | `/{group}/-/members` |
| 添加成员 | POST | `/{group}/-/members/{username}` |
| 更新成员 | PUT | `/{group}/-/members/{username}` |
| 移除成员 | DELETE | `/{group}/-/members/{username}` |

## 另请参阅

- [cnb group](/group/)
- [cnb group view](/group/view)
