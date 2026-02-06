# web-demo

Run the CSR demo with Trunk:

```bash
# from repo root
./scripts/dev-web-demo.sh
```

## Styling overrides

For quick CSS iteration during development, prefer making changes in
`apps/web-demo/dev-overrides.css` (loaded after `app.css`).

Once a styling change is stable, move it into the relevant component `styles.rs`
so it becomes the source of truth (instead of staying as a demo-only override).

Or manually:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli --version 0.2.108

cd apps/web-demo
env -u NO_COLOR trunk serve --open true
```

Quick regression check (blank-screen guard):

```bash
./scripts/smoke-web-demo.sh
```
