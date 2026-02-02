# tauri-demo

Tauri shell for the `apps/web-demo` Leptos CSR frontend.

## Dev

Prerequisites:

- `rustup` + target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`
- Tauri CLI: `cargo install tauri-cli`

Run:

```bash
cd apps/tauri-demo
cargo tauri dev
```

This uses `apps/tauri-demo/tauri.conf.json` and runs `trunk serve` from `apps/web-demo` automatically.

