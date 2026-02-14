# rust-ui

Layered UI primitives + components for Leptos:

- `crates/ui-state-primitives`: state (Stately)
- `crates/ui-headless`: behavior + a11y (Aria)
- `crates/ui-theme`: tokens → CSS variables
- `crates/ui-motion`: spring/WAAPI motion runtime
- `crates/ui-components`: Spectrum-grade components
- `apps/web-demo`: CSR demo app (Trunk)
- `apps/docs-app`: CSR docs site (Trunk)

## Documentation

- Docs entrypoint: `docs/README.md`
- Philosophy and strategy: `docs/philosophy.md`
- Rules (hard constraints): `docs/RULES_ZH.md`
- Full markdown index: `docs/DOCS_INDEX.md`
- Doc governance: `docs/DOCS_GOVERNANCE.md`

## Quick start (recommended)

From the repo root (`rust-ui/`):

```bash
./scripts/dev-web-demo.sh
./scripts/dev-docs-app.sh
```

These scripts:

- ensure `~/.cargo/bin` is on `PATH` (so `trunk` is found)
- unset `NO_COLOR` (Trunk treats it as a boolean env var)
- force `RUSTFLAGS="--cfg erase_components"` (prevents Tachys tuple blowups in WASM)
- check `wasm32-unknown-unknown` + `wasm-bindgen` are installed

## Smoke checks (blank-screen guard)

```bash
./scripts/smoke-web-demo.sh
./scripts/smoke-docs-app.sh
```

If a smoke check fails, it prints the Trunk log path and keeps a screenshot for debugging.

## Full gates

```bash
./scripts/check.sh
```
