# flatpak

## 依赖

- `flatpak`
- `flatpak-builder`

在 Ubuntu/Debian 上安装依赖:

```bash
sudo apt install flatpak flatpak-builder
```

## 使用方法

在项目 `linux/packaging/flatpak` 目录下添加 `make_config.yaml`:

```yaml
app_id: com.example.app
runtime: org.freedesktop.Platform
runtime_version: '23.08'
sdk: org.freedesktop.Sdk
finish_args:
  - --share=network
  - --filesystem=home
```

执行:

```bash
fastforge package --platform linux --targets flatpak
```
