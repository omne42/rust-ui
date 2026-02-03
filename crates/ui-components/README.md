# ui-components

`ui-components` is the final Leptos component layer (React Spectrum analogue).

It composes:

- `ui-core` (state)
- `ui-headless` (interaction + a11y)
- `ui-theme` (tokens → CSS variables)
- `ui-motion` (spring/WAAPI backends)

## Goals

- Public API stays small and stable (v0).
- No direct `web-sys` types in public props.
- Styling is token-driven (`var(--ui-*)`), injected via `<UiRoot>`.
- Motion is physics-first; runtime values flow through CSS variables (`--*`).

## Usage

```rust
use leptos::prelude::*;
use ui_components::{Button, Theme, UiRoot, provide_focus_visible, provide_overlay_stack};

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    let theme = Signal::derive(|| Theme::dark());

    view! {
        <UiRoot theme=theme safe_area=true>
            <Button>"Hello"</Button>
        </UiRoot>
    }
}
```

## Component structure

Most components follow an internal split:

- `logic.rs`: props normalization + composition of headless hooks
- `motion.rs`: motion contract + attaching to the engine
- `styles.rs`: static CSS (token-driven)
- `view.rs`: DOM structure + mounting attrs/handlers

## Tests

```bash
cargo test -p ui-components
```

