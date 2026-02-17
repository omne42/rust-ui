# ui-motion

`ui-motion` provides native motion primitives (Framer/baseline direction), decoupled from components.

## Goals

- Spring-physics runtime for micro-interactions (hover/tap/active highlight/presence).
- Web backend (wasm32) for WAAPI-based keyframe animations.
- Respect `prefers-reduced-motion` (reduce → no-op / snap-to-target).
- Compile on non-wasm targets (SSR/tooling) via no-op stubs.

## API (v0)

- Spring runtime: `ui_motion::spring::{SpringConfig, SpringAnimator}`
- Spring presets: `ui_motion::presets::*`
- Web WAAPI backend (`wasm32`): `ui_motion::web::{prefers_reduced_motion, animate}`

## Example (spring driving CSS variables)

```rust
use ui_motion::spring::{SpringAnimator, SpringConfig};

let _anim = SpringAnimator::new(0.0, SpringConfig::default(), move |v| {
    // e.g. element.style().set_property("--x", &format!("{v}px"));
});
```

## Tests

```bash
cargo test -p ui-motion
```

