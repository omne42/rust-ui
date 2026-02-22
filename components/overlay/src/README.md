# Overlay

`Overlay` is a portal-based dialog surface built from `ui-headless` + `ui-motion` + `ui-theme`.

## Goals / Non-goals / Risk Boundary

- Goal: provide accessible, testable, and predictable overlay behavior (focus trap, dismiss contracts, source markers).
- Non-goal: do not implement app-level business state, routing, or async orchestration inside the component.
- Risk boundary: if interaction or accessibility semantics drift, fix in headless/core boundaries first, not by patching random branches in `view.rs`.

## Architecture Layers

- `logic.rs`: normalize optional text, derive source/state markers, and centralize Escape-close guard logic.
- `view.rs`: render portal/backdrop/panel structure and mount headless contracts (`use_modal`, `use_focus_trap`, overlay stack registration).
- `motion.rs`: `OverlayMotion` contract + wasm spring attach; SSR/non-wasm fallback is no-op with immediate exit completion.
- `styles.rs`: static CSS contract driven by semantic markers (`data-*`) and CSS vars.
- `mod.rs`: stable public exports (`Overlay`, `OverlayMotion`, and part-state contracts).

## API (Table)

### Overlay Props

| Prop | Type | Default |
| --- | --- | --- |
| `open` | `Signal<bool>` | required |
| `on_close` | `OnPress` | required |
| `children` | `ChildrenFn` | required |
| `aria_labelledby` | `Option<String>` | `None` |
| `aria_describedby` | `Option<String>` | `None` |
| `role` | `&'static str` | `"dialog"` |
| `is_dismissable` | `bool` | `true` |
| `is_keyboard_dismiss_disabled` | `bool` | `false` |
| `motion` | `OverlayMotion` | `OverlayMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |

### Overlay Events

| Event | Type | Default |
| --- | --- | --- |
| `on_close` | `OnPress` | required |
| `on_exit_complete` | `Callback<()>` | optional |

## Hello World (Minimum Viable)

```rust
let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

let (present, set_present) = signal(open.get_untracked());
Effect::new(move |_| {
  if open.get() {
    set_present.set(true);
  }
});

let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let on_exit_complete = Callback::new(move |_| set_present.set(false));

<Show when=move || present.get()>
  <Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>
    <div>"Overlay content"</div>
  </Overlay>
</Show>
```

- Presence (`Show when=present`) is the recommended default path so close animation can finish before unmount.
- Basic usage does not require users to manually wire low-level state primitives.

## Semantics and Accessibility

- Panel uses `role` + `aria-modal="true"`, with optional `aria-labelledby` and `aria-describedby`.
- Focus is trapped in panel via `use_focus_trap`.
- Escape closes only when all guard conditions pass: topmost overlay, not composing IME input, not default-prevented, and keyboard dismiss enabled.
- Stable markers are exposed for testing and automation:
  - root: `data-state`, `data-dismiss`, `data-keyboard-dismiss`, `data-*-source`
  - parts: `data-slot="overlay|overlay-backdrop|overlay-panel"`

## Motion and Fallback

- `OverlayMotion` contains spring config + entry pose (`initial_scale`, `initial_y_px`).
- Runtime sanitization clamps/normalizes invalid custom motion values (`sanitize_motion`).
- wasm path uses `ui-motion::spring::SpringAnimator`; non-wasm path is deterministic no-op for SSR/tooling builds.

## Tree Shaking and Feature Gate

- Component feature: `component-overlay`.
- CSS injection is gated via `inject-css` and `push_components_css` conditional aggregation.
- Public export is gated from `crates/ui/src/lib.rs`.

## Test Contract

- Semantic contract tests live in `components/overlay/test/overlay_semantics.rs`.
- Coverage includes module exports, marker contracts, Escape behavior guards, motion contract sanitization, CSS aggregation, and docs playground anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/overlays.rs`
- `overlay()` section includes:
  - `Overlay presence`
  - `State + Source Markers`

## Source-first Copy-Paste Ready

- Real source files:
  - `components/overlay/src/mod.rs`
  - `components/overlay/src/logic.rs`
  - `components/overlay/src/view.rs`
  - `components/overlay/src/styles.rs`
  - `components/overlay/src/motion.rs`
- docs-app playground snippets are aligned with these source contracts.
