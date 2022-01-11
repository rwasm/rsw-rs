# rsw-rs

> This project is in early experimental stage.

```bash
# dev
rsw watch

# release
rsw build
```

```toml
# rsw.toml
name = 'rsw-config'
version = "0.0.1"

[[crates]]
name = "wasm-foo"
out_dir = "./outdir"

[[crates]]
name = "@rsw/foo"
```
