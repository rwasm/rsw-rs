<p align="center">
  <img src="./rsw.png" width="120">
</p>

「🌐 Language」[简体中文](./README.zh_CN.md)

## rsw-rs

> This project is in early experimental stage.

## Usage

```bash
cargo install rsw
```

```bash
# help
rsw -h

# dev
rsw watch

# release
rsw build
```

```toml
# rsw.toml
name = "rsw"
version = "0.0.1"
# default `50` ms
interval = 50

# ---------------------------

# package: rsw-foo
[[crates]]
# default `.`
root = "."
# npm package name
name = "rsw-foo"
# default `pkg`
out-dir = "mypkg"
[crates.watch]
# default `true`
run = false
# profile: `dev` | `profiling`, defalut `dev`
profile = "dev"
[crates.build]
run = false
# profile: `release` | `profiling`, default `release`
profile = "profiling"

# ---------------------------

# package: @rsw/bar
[[crates]]
name = "@rsw/bar"
```

## Feature

- rsw watch
- rsw build

## TODO

- rsw init - `rsw.toml`
- rsw new - rust crate
- deps watch - local sub-dependency file changes trigger hot updates

## License

MIT License © 2022 lencx
