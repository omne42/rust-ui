# ui-visual-primitive

`ui-visual-primitive` stores internal visual primitives that are shared by
`ui` and `ui-layout`.

## Positioning

- This crate is **not** a public component catalog for normal users.
- It hosts reusable visual-geometry primitives (DOM measurement + CSS variable
  driving) that do not belong to business components.
- `ui-layout` remains a user-facing package and is not merged into this crate.

## Current primitives

- `active_highlight`: shared active-row highlight style + motion driver.
- `ripple`: shared ripple motion contract + trigger helpers.

## Dependency boundary

- May depend on `leptos` + `ui-motion`.
- Must not carry business semantics or component-specific behavior.
- Must avoid `unsafe`.
