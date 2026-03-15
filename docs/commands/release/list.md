# cnb-rs release list

```
cnb-rs release list
```

列出仓库的 Release（最多 100 条）。

输出为表格格式，包含 Release 名称、Tag 名称、类型标记（`Latest` / `Pre release`）和发布时间。

## 选项

**继承的全局选项：**

- `--repo <REPO>`: 指定仓库路径（格式：`group/repo`）
- `--json`: 以 JSON 格式输出
- `--domain <DOMAIN>`: 指定目标域名（默认：`cnb.cool`）

## 输出示例

```
Name            TAG NAME        TYPE            PUBLISHED
v1.2.0          v1.2.0          Latest          2024-01-15 10:30:00
v1.1.0-beta     v1.1.0-beta     Pre release     2024-01-10 08:00:00
```

## 另请参阅

- [cnb-rs release](/release/)
