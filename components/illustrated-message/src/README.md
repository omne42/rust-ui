# IllustratedMessage

`IllustratedMessage` is an empty-state display component with optional illustration, description, and action slots.

## Goals / Non-goals / Risk Boundary

- Goal: provide a composable empty-state surface with simple defaults and deterministic slot rendering.
- Non-goal: do not manage fetching, retries, or page-level state transitions.
- Risk boundary: slot visibility rules remain centralized in `logic.rs` (`resolve_view_state`).

## Architecture Layers

- `logic.rs`: derive `show_*` flags from optional text/slot presence.
- `view.rs`: render slot layout based on resolved view state.
- `motion.rs`: `IllustratedMessageMotion` contract, sanitize custom values, wasm attach with non-wasm fallback.
- `styles.rs`: token-first static CSS.
- `mod.rs`: stable exports (`IllustratedMessage`, `IllustratedMessageMotion`, orientation enum).

## API (Table)

### IllustratedMessage Props

| Prop | Type | Default |
| --- | --- | --- |
| `title` | `Option<String>` | `None` |
| `description` | `Option<String>` | `None` |
| `illustration` | `Option<ViewFn>` | `None` |
| `actions` | `Option<ViewFn>` | `None` |
| `orientation` | `IllustratedMessageOrientation` (`Vertical` / `Horizontal`) | `Vertical` |
| `motion` | `IllustratedMessageMotion` | `IllustratedMessageMotion::default()` |
| `class_name` | `Option<String>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | display-only primitive | `-` |

## Hello World (Minimum Viable)

```rust
<IllustratedMessage
  title="No results".to_string()
  description="Try changing your search.".to_string()
  illustration=move || view! { <div class="docs-illustration">"o"</div> }
  actions=move || view! { <ui_components::Button>"Clear"</ui_components::Button> }
/>
```

## Semantics and Rendering Contract

- Root marker: `data-slot="illustrated-message"`.
- Optional slots are rendered only when corresponding `show_*` flags are true:
  - `illustrated-message-illustration`
  - `illustrated-message-title`
  - `illustrated-message-description`
  - `illustrated-message-actions`
- Empty or whitespace-only `title`/`description` is treated as absent.

## Motion and Fallback

- Default motion uses `ui_motion::presets::spring_soft()` with `initial_y_px=8.0`.
- Custom motion is sanitized (`spring` validity + `initial_y_px` clamped to `[0, 120]`).
- Non-wasm path is safe no-op to keep SSR/tooling builds deterministic.

## Test Contract

- Semantic tests: `components/illustrated-message/tests/illustrated_message_semantics.rs`.
- Coverage includes slot toggles, orientation contract, motion sanitization, and docs anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/display.rs`
- `illustrated_message()` includes an `Empty state` playground.

## Source-first Copy-Paste Ready

- Source files:
  - `components/illustrated-message/src/mod.rs`
  - `components/illustrated-message/src/logic.rs`
  - `components/illustrated-message/src/view.rs`
  - `components/illustrated-message/src/styles.rs`
  - `components/illustrated-message/src/motion.rs`
