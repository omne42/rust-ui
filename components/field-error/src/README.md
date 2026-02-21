# FieldError

`FieldError` is a baseline-style form error primitive with centralized tone/visibility/message normalization.

## Start Here (先用起来，再进阶)

1. Copy `Hello World (Minimum Viable)` first; it works without wiring low-level state/headless primitives.
2. Use `Common Usage` to cover the most frequent visible/disabled/tone states.
3. Read `Advanced Controls` only when you need legacy alias compatibility or source-marker details.

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
| `is_visible` | `Option<bool>` | `None` (`visible` fallback, then `false`) |
| `is_disabled` | `Option<bool>` | `None` (`disabled` fallback, then `false`) |
| `is_icon_visible` | `Option<bool>` | `None` (`show_icon` fallback, then `false`) |
| `visible` | `bool` (legacy alias) | `false` |
| `disabled` | `bool` (legacy alias) | `false` |
| `show_icon` | `bool` (legacy alias) | `false` |
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
  is_visible=true
  message="Email is required".to_string()
/>
```

## Common Usage

```rust
<FieldError is_visible=true message="Email is required".to_string() />
<FieldError is_visible=true tone=FieldErrorTone::Neutral message="Use 12+ chars".to_string() />
<FieldError is_visible=true is_disabled=true is_icon_visible=true message="Read-only error".to_string() />
```

## Semantics and Accessibility

- Root exposes stable markers: `data-tone`, `data-state`, `data-visible`, `data-disabled`, `data-message-source`, `data-aria-source`, `data-class-source`.
- Visible message node uses `role="alert"`.
- Hidden state exports `aria-hidden="true"` for predictable assistive behavior.

## Advanced Controls (When Needed)

- `tone=Auto` resolves to `Negative` when visible, otherwise `Neutral`.
- Message fallback only applies when `is_visible=true` (or legacy `visible=true`).
- Naming migration: `is_*` props are preferred; when both forms are provided, `is_*` wins.
- Blank/whitespace custom text is normalized away before state resolution.

## Test Contract

- Semantic tests: `crates/ui-components/tests/field_error_semantics.rs`.
- Coverage includes tone mapping, source markers, hidden/disabled behavior, and docs anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/forms_extra.rs`
- `field_error()` includes:
  - `Hello World (Snapshot Baseline)`
  - `State Matrix (Visible / Hidden / Disabled)`
  - `Controlled vs Uncontrolled (Stateless Contract)`
  - `Visible + Tone`
  - `Hidden + Disabled + Custom Class`

## Source-first Copy-Paste Ready

- Source files:
  - `components/field-error/src/mod.rs`
  - `components/field-error/src/logic.rs`
  - `components/field-error/src/view.rs`
  - `components/field-error/src/styles.rs`
