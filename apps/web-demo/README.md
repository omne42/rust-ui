# web-demo

Run the CSR demo with Trunk:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

cd apps/web-demo
trunk serve --open
```

