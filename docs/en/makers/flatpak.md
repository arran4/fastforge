# flatpak

## Requires

- `flatpak`
- `flatpak-builder`

Install requirements on Ubuntu/Debian:

```bash
sudo apt install flatpak flatpak-builder
```

## Usage

Add `make_config.yaml` to your project `linux/packaging/flatpak` directory.

```yaml
app_id: com.example.app
runtime: org.freedesktop.Platform
runtime_version: '23.08'
sdk: org.freedesktop.Sdk
finish_args:
  - --share=network
  - --filesystem=home
```

Run:

```bash
fastforge package --platform linux --targets flatpak
```
