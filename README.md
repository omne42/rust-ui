# rust-ui

Layered UI primitives + components for Leptos:

- `crates/ui-state-primitives`: state (Stately)
- `crates/ui-headless`: behavior + a11y (Aria)
- `crates/ui-theme`: tokens → CSS variables
- `crates/ui-motion`: spring/WAAPI motion runtime
- `crates/ui`: Spectrum-grade components
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
- auto-patch `trunk` once so browser `Build failure` overlays include cargo stderr details

To skip the auto-patch step:

```bash
TRUNK_PATCH_ERROR_DETAILS=0 ./scripts/dev-docs-app.sh
```

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

## Production static builds (compressed + cache headers)

```bash
./scripts/release-web-demo.sh
./scripts/release-docs-app.sh
```

These wrappers run `trunk build --release`, generate `.gz` (and `.br` if `brotli` is installed)
for `wasm/js/css`, and write `dist/_headers` with immutable cache headers for hashed assets.
