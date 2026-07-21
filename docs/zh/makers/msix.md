# msix

## 用法

将 `make_config.yaml` 添加到你的项目 `windows/packaging/msix` 目录。

你也可以将 `make_config.yaml` 添加到你的项目 `windows/packaging` 目录，以加载默认配置。

```yaml
display_name: HelloWorld
msix_version: 1.0.0.0
# logo_path: C:\path\to\logo.png
```

> 查看所有配置：[msix](https://github.com/YehudaKremer/msix)

运行：

```
fastforge package --platform windows --targets msix
```

## 相关链接

https://github.com/YehudaKremer/msix
