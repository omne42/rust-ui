# ui-state-primitives

`ui-state-primitives` provides platform-agnostic UI state primitives (React Stately analogue).

## Goals

- Pure state + state machines (controlled/uncontrolled, selection, collections, overlay triggers).
- No DOM, no `web-sys`, no styling, no animation.
- Unit-testable on any target.

## API surface (v0)

- Controlled/uncontrolled helper: `ui_state_primitives::controlled::use_controlled_state`
- Toggle: `ui_state_primitives::toggle::use_toggle_state`
- Selection: `ui_state_primitives::selection::{use_single_selection_state, use_multiple_selection_state}`
- List modeling: `ui_state_primitives::list::use_list_state`
- Overlay open/close modeling: `ui_state_primitives::overlay_trigger::use_overlay_trigger_state`

## Example

```rust
use ui_state_primitives::toggle::{ToggleStateOptions, use_toggle_state};

let mut state = use_toggle_state(ToggleStateOptions {
    default_selected: Some(false),
    ..Default::default()
});

state.toggle();
assert!(state.is_selected());
```

## Tests

```bash
cargo test -p ui-state-primitives
```
