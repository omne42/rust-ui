# FieldError

`FieldError` is a baseline-style form error primitive with centralized tone/visibility/message normalization.

## Goals / Non-goals / Risk Boundary

- Goal: expose predictable error semantics and source markers for forms.
- Non-goal: do not own field validation logic or async retry workflow.
- Risk boundary: visibility/message fallback must stay centralized in `logic.rs`, not duplicated across views.

## Architecture Layers

- `logic.rs`: normalize aria/message/class inputs, resolve effective tone and marker sources.
- `view.rs`: render structure and semantic attributes from resolved state.
- `styles.rs`: token-first static CSS.
- `mod.rs`: stable exports (`FieldError`, `FieldErrorTone`, defaults).

## API (Table)

### FieldError Props

| Prop | Type | Default |
| --- | --- | --- |
| `tone` | `FieldErrorTone` (`Auto` / `Neutral` / `Negative`) | `Auto` |
| `visible` | `bool` | `false` |
| `disabled` | `bool` | `false` |
| `show_icon` | `bool` | `false` |
| `message` | `Option<String>` | `None` (`"Invalid value"` when visible and empty) |
| `aria_label` | `Option<String>` | `None` (`"FieldError"` fallback) |
| `class_name` | `Option<String>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | stateless display primitive | `-` |

## Hello World (Minimum Viable)

```rust
<FieldError
  visible=true
  message="Email is required".to_string()
/>
```

## Semantics and Accessibility

- Root exposes stable markers: `data-tone`, `data-state`, `data-visible`, `data-disabled`, `data-message-source`, `data-aria-source`, `data-class-source`.
- Visible message node uses `role="alert"`.
- Hidden state exports `aria-hidden="true"` for predictable assistive behavior.

## Behavior Notes

- `tone=Auto` resolves to `Negative` when visible, otherwise `Neutral`.
- Message fallback only applies when `visible=true`.
- Blank/whitespace custom text is normalized away before state resolution.

## Test Contract

- Semantic tests: `crates/ui-components/tests/field_error_semantics.rs`.
- Coverage includes tone mapping, source markers, hidden/disabled behavior, and docs anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/forms_extra.rs`
- `field_error()` includes:
  - `Visible + Tone`
  - `Hidden + Disabled + Custom Class`

## Source-first Copy-Paste Ready

- Source files:
  - `crates/ui-components/src/field_form/field_error/mod.rs`
  - `crates/ui-components/src/field_form/field_error/logic.rs`
  - `crates/ui-components/src/field_form/field_error/view.rs`
  - `crates/ui-components/src/field_form/field_error/styles.rs`
