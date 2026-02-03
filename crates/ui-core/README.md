# ui-core

`ui-core` provides platform-agnostic UI state primitives (React Stately analogue).

## Goals

- Pure state + state machines (controlled/uncontrolled, selection, collections, overlay triggers).
- No DOM, no `web-sys`, no styling, no animation.
- Unit-testable on any target.

## API surface (v0)

- Controlled/uncontrolled helper: `ui_core::controlled::use_controlled_state`
- Toggle: `ui_core::toggle::use_toggle_state`
- Selection: `ui_core::selection::{use_single_selection_state, use_multiple_selection_state}`
- List modeling: `ui_core::list::use_list_state`
- Overlay open/close modeling: `ui_core::overlay_trigger::use_overlay_trigger_state`

## Example

```rust
use ui_core::toggle::{ToggleStateOptions, use_toggle_state};

let mut state = use_toggle_state(ToggleStateOptions {
    default_selected: Some(false),
    ..Default::default()
});

state.toggle();
assert!(state.is_selected());
```

## Tests

```bash
cargo test -p ui-core
```

