# AvatarGroup

`AvatarGroup` composes `Avatar` items into an overflow-aware group with stable semantic markers.

## Layering

- `logic.rs`: normalization and group render-state derivation.
- `view.rs`: Leptos render + headless group a11y wiring.
- `styles.rs`: token-first static CSS.
- `mod.rs`: minimal public exports.
