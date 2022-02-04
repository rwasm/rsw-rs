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
name = 'rsw'
version = "0.0.1"
interval = 50

[[crates]]
root = "."
name = "rsw-foo"
# out-dir = "./mypkg"
# profile: dev | profiling, defalut `dev`
[crates.watch]
run = true
profile = "dev"
# profile: release | profiling, default `release`
[crates.build]
run = false
profile = "profiling"

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
