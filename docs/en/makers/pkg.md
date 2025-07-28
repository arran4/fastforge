# pkg

> You can only build the PKG target on macOS machines.

## Usage

Add `make_config.yaml` to your project `macos/packaging/pkg` directory.
Defaults may also come from `macos/packaging/make_config.yaml`.

```yaml
install-path: /Applications
sign-identity: <your-sign-identity>
```

Run:

```
fastforge package --platform macos --targets pkg
```

## Related Links

- [Build and release a macOS app](https://docs.flutter.dev/deployment/macos)
