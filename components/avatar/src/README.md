# Avatar

`Avatar` is a display component that renders an image when available and falls back to initials when the image is missing or fails.

## Quick Start (Hello World)

Use the default API first. No state machine wiring is required.

```rust
use leptos::prelude::*;
use ui::Avatar;

view! {
    <Avatar />
}
```

## Common Usage

### 1) Name + image (automatic fallback)

```rust
use leptos::prelude::*;
use ui::{Avatar, AvatarSize};

view! {
    <Avatar
        name="Ada Lovelace".to_string()
        src="https://example.com/avatar.png".to_string()
        size=AvatarSize::Md
    />
}
```

### 2) Name only

```rust
use leptos::prelude::*;
use ui::Avatar;

view! {
    <Avatar name="Grace Hopper".to_string() />
}
```

### 3) Size variants

`size` uses `AvatarSize::{Sm, Md, Lg}` and defaults to `AvatarSize::Md`.

## Advanced Options

- `alt`: overrides label source priority (`alt -> name -> fallback`).
- `lang`/`dir`: attach locale direction semantics.
- `class_name`: attach custom class after normalization.

These options are optional. Start with the default call path above.

## LLM Output Boundary

- Streaming: Optional (not required for avatar).
- Fallback mode: `fallback=snapshot` (avatar only consumes complete props snapshots).
- Output lifecycle status (`draft`/`verified`/`submittable`) is owned by parent message/output containers.
- Avatar keeps continuous semantic markers (`role`/`aria-*`/`data-*`) regardless of upstream output mode.

## Layering

- `logic.rs`: input normalization and state derivation.
- `view.rs`: Leptos render + headless a11y mounting.
- `styles.rs`: token-first static CSS.
- `mod.rs`: minimal public exports.
