# web-demo

Run the CSR demo with Trunk:

```bash
# from repo root
./scripts/dev-web-demo.sh
```

Or manually:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli --version 0.2.108

cd apps/web-demo
env -u NO_COLOR trunk serve --open true
```
