# ActionBar

ActionBar is a bulk-action surface. It stays hidden when no items are selected,
and becomes visible when selection exists.

## Start Here (Hello World)

Use the default API first. No internal state machine wiring is required.

```rust
use leptos::prelude::*;
use ui::{ActionBar, ActionButton};

view! {
    <ActionBar default_selected_count=1>
        <ActionButton>"Archive"</ActionButton>
    </ActionBar>
}
```

## Common Usage

### 1) Controlled

```rust
use leptos::prelude::*;
use ui::{ActionBar, ActionButton};

let (selected_count, set_selected_count) = signal(2_usize);
let selected_count_signal = Signal::derive(move || selected_count.get());
let on_selected_count_change = Callback::new(move |next: usize| set_selected_count.set(next));
let on_clear_selection = Callback::new(move |_| set_selected_count.set(0));

view! {
    <ActionBar
        selected_count=selected_count_signal
        on_selected_count_change=on_selected_count_change
        on_clear_selection=on_clear_selection
    >
        <ActionButton>"Delete"</ActionButton>
        <ActionButton is_quiet=true>"Archive"</ActionButton>
    </ActionBar>
}
```

### 2) Uncontrolled

```rust
use leptos::prelude::*;
use ui::{ActionBar, ActionButton};

view! {
    <ActionBar default_selected_count=2>
        <ActionButton>"Tag"</ActionButton>
        <ActionButton is_quiet=true>"Assign"</ActionButton>
    </ActionBar>
}
```

## Learn In Order

1. Start with `default_selected_count` (default path).
2. Add `on_clear_selection` when you need clear behavior.
3. Switch to controlled mode with `selected_count + on_selected_count_change` only when needed.
4. Use advanced props (`position`, `is_force_visible`, `selection_text`, `clear_label`, `motion`) last.

## Docs Entry

- docs-app page: `apps/docs-app/src/pages/components/pages/actions_extra.rs` (`action_bar`)
- live route: `#/components/action-bar`
