# rsw-rs

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
out_dir = "./outdir"

[[crates]]
name = "@rsw/foo"
```

## Feature

- debug
- watch
- build
- new
