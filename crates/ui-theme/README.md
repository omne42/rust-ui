# ui-theme

`ui-theme` is the design-token layer (Baseline theme analogue).

It owns:

- Token taxonomy (palette/semantic/aliases/component/layout/typography)
- Theme context axes (`system/color/scale`) + presets (Light/Dark/OLED)
- CSS variable emission

## Goals

- Tokens are an interface: components consume `var(--ui-*)`, not hard-coded colors.
- Color format uses **OKLCH**, with an **OLED** preset for true-black backgrounds.
- Theme mapping is centralized: `tokens.rs` defines, `theme.rs` maps, `css.rs` emits.
- No component CSS here (that lives in `ui`).

## Usage

```rust
use ui_theme::{Theme, ThemeColor, ThemeScale};

let css = Theme::oled().to_css_variables();

let large_s2 = Theme::baseline_two(ThemeColor::Dark, ThemeScale::Large);
let large_css = large_s2.to_css_variables();
```

The demo uses `ui::UiRoot`, which injects theme variables + component CSS automatically.

## Tests

```bash
cargo test -p ui-theme
```
