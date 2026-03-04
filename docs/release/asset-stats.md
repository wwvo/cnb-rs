# cnb release asset-stats

```
cnb release asset-stats
```

统计所有 Release 的附件大小。

遍历仓库所有 Release，列出每个包含附件的 Release 的名称、Tag、附件总大小和发布时间，最后输出所有附件的总大小。附件大小自动转换为可读格式（B/KB/MB/GB/TB）。

## 输出示例

```
Name            TAG NAME        ASSET SIZE           PUBLISHED
v1.2.0          v1.2.0          15.3 MB              2024-01-15 10:30:00
v1.1.0          v1.1.0          12.8 MB              2024-01-10 08:00:00
Total Size: 29456384 (28.1 MB)
```

## 另请参阅

- [cnb release](/release/)
