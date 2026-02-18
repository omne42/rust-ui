# Resizable

`Resizable` is a two-panel splitter for layout surfaces.

## Hello World

```rust
<Resizable
  orientation=ResizableOrientation::Horizontal
  first=move || view! { <div>"Sidebar"</div> }
  second=move || view! { <div>"Content"</div> }
/>
```

## Controlled Split

```rust
let (split_raw, set_split_raw) = signal(58.0_f64);
let split: Signal<f64> = Signal::derive(move || split_raw.get());

<Resizable
  orientation=ResizableOrientation::Vertical
  value=split
  on_value_change=Callback::new(move |next| set_split_raw.set(next))
  min_split_percent=25.0
  max_split_percent=80.0
  first=move || view! { <div>"Header"</div> }
  second=move || view! { <div>"Body"</div> }
/>
```

## API Notes

- Canonical value axis: `value + on_value_change + default_value`.
- Legacy aliases are still accepted: `split_percent + on_split_percent_change + default_split_percent`.
- `is_disabled` and `is_with_handle` follow the global `is_*` naming contract.
- Interaction and a11y semantics are delegated to `ui-headless`.
- State invariants are delegated to `ui-state-primitives`.
