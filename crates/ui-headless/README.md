# ui-headless

`ui-headless` provides interaction + accessibility primitives (无障碍基线 analogue).

It outputs **handlers + attrs** structs, which `ui` (or your app) explicitly mounts onto DOM elements.

## Goals

- Normalize input (pointer/keyboard), focus-visible modality, roving tabindex, aria-* semantics.
- No styling, no motion orchestration.
- Feature-gated for Web/SSR.

## Features

- `default = ["web"]`
- `web`: browser-oriented behavior (CSR); wasm-only global listeners are behind `cfg(target_arch = "wasm32")`
- `ssr`: compile-only / no-op behavior for server/tooling builds (`web` and `ssr` are mutually exclusive)

## Typical usage

At app root (inside a Leptos component):

```rust
use ui_headless::{provide_focus_visible, provide_overlay_stack};

provide_focus_visible();
provide_overlay_stack();
```

Inside a component:

```rust
use leptos::prelude::*;
use ui_headless::{ButtonOptions, use_button};

let aria = use_button(ButtonOptions::default());

view! {
  <button
    role=aria.attrs.role
    tabindex=aria.attrs.tabindex
    aria-disabled=aria.attrs.aria_disabled
    on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
    on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
    on:click=move |_| aria.handlers.press.on_click.run(())
  />
}
```

## Tests

```bash
cargo test -p ui-headless
```

