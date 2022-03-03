# @rsw/cli

## Pre-installed

- [rust](https://www.rust-lang.org/learn/get-started)
- [nodejs](https://nodejs.org)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

## Quick start

```bash
# https://github.com/lencx/create-mpl
# npm 6.x
npm init mpl@latest my-app --type wasm

# npm 7+, extra double-dash is needed:
npm init mpl@latest my-app -- --type wasm
```

---

## Usgae

```bash
# install
npm install -g @rsw/cli
```

```bash
# help
rsw -h

# rsw.toml - initial configuration
rsw init

# generate a wasm project
rsw new <name>

# dev mode
rsw watch

# release mode
rsw build

# clean - link & build
rsw clean
```

[rsw docs](https://github.com/lencx/rsw-rs/blob/main/README.md)
