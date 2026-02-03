# ui-theme

`ui-theme` is the design-token layer (Spectrum theme analogue).

It owns:

- Token structs (colors/radius/space/shadow)
- Theme presets (Light/Dark/OLED)
- CSS variable emission

## Goals

- Tokens are an interface: components consume `var(--ui-*)`, not hard-coded colors.
- Color format uses **OKLCH**, with an **OLED** preset for true-black backgrounds.
- No component CSS here (that lives in `ui-components`).

## Usage

```rust
use ui_theme::Theme;

let css = Theme::oled().to_css_variables();
```

The demo uses `ui-components::UiRoot`, which injects theme variables + component CSS automatically.

## Tests

```bash
cargo test -p ui-theme
```

