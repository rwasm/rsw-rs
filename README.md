<p align="center">
  <img src="./rsw.png" width="120">
  <h3>rsw-rs</h3>
</p>

> This project is in early experimental stage.

```bash
# dev
rsw watch

# release
rsw build

# create crate
rsw new
```

```toml
# rsw.toml
name = 'rsw'
version = "0.0.1"

[[crates]]
name = "rsw-foo"
out-dir = "./mypkg"
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

- debug
- watch
- build
- new
