# IllustratedMessage

`IllustratedMessage` is an empty-state display component with optional illustration, description, and action slots.

## Start Here (Hello World)

Use the default path first. No state wiring is required.

```rust
<IllustratedMessage
  title="Empty".to_string()
  description="Nothing here".to_string()
/>
```

## Common Usage

Add optional slots when you need richer empty-state presentation.

```rust
<IllustratedMessage
  title="No results".to_string()
  description="Try changing your search.".to_string()
  illustration=move || view! { <div class="docs-illustration">"o"</div> }
  actions=move || view! { <ui::Button>"Clear"</ui::Button> }
/>
```

## Advanced Options (Optional)

Use these only when the default path is not enough:

- custom `orientation` (`Vertical` / `Horizontal`)
- custom `motion` contract (`IllustratedMessageMotion`)
- locale context (`lang` / `dir`)
- custom class hook (`class_name`)

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
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<ui_headless::A11yDirection>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | display-only primitive | `-` |

## Semantics and Rendering Contract

- Root marker: `data-slot="illustrated-message"`.
- Optional slots are rendered only when corresponding `show_*` flags are true:
  - `illustrated-message-illustration`
  - `illustrated-message-title`
  - `illustrated-message-description`
  - `illustrated-message-actions`
- Empty or whitespace-only `title`/`description` is treated as absent.
- Agent contract markers are schema-typed and machine-readable:
  - `data-ui-schema` / `data-ui-schema-version`
  - `data-ui-intent` / `data-ui-action`
  - `data-ui-state` / `data-ui-source`
  - `data-ui-config-policy="whitelist"`
  - `data-ui-streaming-policy="optional"` / `data-ui-streaming-fallback="snapshot"`
  - `data-ui-output-status="validated"`

## Motion and Fallback

- Default motion uses `ui_motion::presets::spring_soft()` with `initial_y_px=8.0`.
- Custom motion is sanitized (`spring` validity + `initial_y_px` clamped to `[0, 120]`).
- Non-wasm path is safe no-op to keep SSR/tooling builds deterministic.

## Test Contract

- Semantic tests: `components/illustrated-message/test/semantics.rs`.
- Coverage includes slot toggles, orientation contract, motion sanitization, and docs anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/display.rs`
- `illustrated_message()` includes beginner-first docs sections:
  - `Hello World (Default API)`
  - `State Matrix`
  - `Controlled vs Uncontrolled (N/A)`
  - `Streaming Optional / Snapshot`
  - `Source-first Starter (Copy-Paste Ready)`

## Source-first Copy-Paste Ready

- Source files:
  - `components/illustrated-message/src/Component.toml`
  - `components/illustrated-message/src/illustrated_message.rbi`
  - `components/illustrated-message/src/mod.rs`
  - `components/illustrated-message/src/logic.rs`
  - `components/illustrated-message/src/view.rs`
  - `components/illustrated-message/src/styles.rs`
  - `components/illustrated-message/src/motion.rs`
