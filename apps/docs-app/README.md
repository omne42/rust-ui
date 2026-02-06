# docs-app

Local documentation site for `rust-ui` (CSR via Trunk).

```bash
# from repo root
./scripts/dev-docs-app.sh
```

Common pitfalls:

- `trunk: command not found`: your shell didn't source Cargo env. Either run the script above
  (it prepends `~/.cargo/bin`), or run `source ~/.cargo/env`.
- WASM blank screen / Tachys tuple limits: this repo requires `cfg(erase_components)` for WASM.
  The script sets it for you; running `trunk serve` from the workspace root also picks up
  `.cargo/config.toml`.

Quick regression check:

```bash
./scripts/smoke-docs-app.sh
```
