# dmg

> You can only build the DMG target on macOS machines.

## Requirements

- `appdmg`

Run the following command

```
npm install -g appdmg
```

## Usage

Add `make_config.yaml` to your project `macos/packaging/dmg` directory.
You can also create a `macos/packaging/make_config.yaml` with shared defaults.

```yaml
title: hello_world
contents:
  - x: 448
    y: 344
    type: link
    path: '/Applications'
  - x: 192
    y: 344
    type: file
    path: hello_world.app
```

Run:

```
fastforge package --platform macos --targets dmg
```

## Related Links

- [https://github.com/LinusU/node-appdmg](https://github.com/LinusU/node-appdmg)
- [Build and release a macOS app](https://docs.flutter.dev/deployment/macos)
