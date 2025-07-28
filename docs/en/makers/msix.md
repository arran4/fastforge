# msix

## Usage

Add `make_config.yaml` to your project `windows/packaging/msix` directory.
Common settings can be placed in `windows/packaging/make_config.yaml`.

```yaml
display_name: HelloWorld
msix_version: 1.0.0.0
# logo_path: C:\path\to\logo.png
```

> View all configuration: [msix](https://github.com/YehudaKremer/msix)

Run:

```
fastforge package --platform windows --targets msix
```

## Related Links

https://github.com/YehudaKremer/msix
