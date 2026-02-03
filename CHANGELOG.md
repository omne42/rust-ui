# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

### Added

- Cargo workspace scaffold with layered crates (`ui-core`, `ui-headless`, `ui-theme`, `ui-components`) and demo apps (`web-demo`, `tauri-demo`).
- `ui-core`: initial headless state primitive `use_toggle_state` with unit tests.
- `ui-core`: `use_overlay_trigger_state` (open/close/toggle + controlled/uncontrolled) with unit tests.
- `ui-core`: `use_controlled_state` helper (value/default + on_change pattern) for building controlled/uncontrolled primitives.
- `ui-core`: `use_single_selection_state` / `use_multiple_selection_state` for selection modeling.
- `ui-core`: `use_list_state` (items + selection) for list-based components.
- `ui-theme`: design tokens + CSS variable emitter, plus base/safe-area CSS helpers and unit tests.
- `ui-headless`: initial interaction primitives (focus-visible modality, press handling, button behavior) with `web`/`ssr` feature gating.
- `ui-headless`: `use_hover` (hover state + handlers).
- `ui-headless`: `use_focus_within` (container focus tracking).
- `ui-headless`: `use_roving_tabindex` (roving tabindex state + handlers).
- `ui-headless`: `use_listbox` (aria-activedescendant listbox semantics).
- `ui-headless`: `use_menu` (menu semantics + aria-activedescendant + keyboard navigation/activation).
- `ui-headless`: `use_menu` / `use_listbox` typeahead navigation when `item_text` is provided.
- `ui-headless`: `use_popover_position` (anchor-rect positioning; fixed layout).
- `ui-components`: initial `<Button>` component integrating headless behavior + theme tokens.
- `ui-components`: `ListBox` (v0 demo component built on `use_listbox`).
- `ui-components`: `Popover` (v0 positioned popover using headless positioning + overlay stack + focus trap).
- `ui-components`: `Menu` / `MenuTrigger` (v0 popover-based menu composition).
- `ui-components`: `Select` (v0 Button + Popover + ListBox composition).
- `ui-components`: `Menu` / `ListBox` / `Select` provide `item_text` to enable typeahead navigation.
- `ui-headless`: `use_checkbox` / `use_switch` (role + `aria-checked` + keyboard press handling via `PressActivationKeys`).
- `ui-components`: `<Checkbox>` / `<Switch>` components (built on headless press + focus ring).
- `ui-headless`: overlay primitives (topmost overlay stack + focus trap with focus restore).
- `ui-headless`: `use_focus_ring` hook for per-component focus ring handling.
- `ui-components`: initial `<Overlay>` (portal + Esc/topmost + click-outside + focus trap).
- `apps/web-demo`: minimal Leptos CSR demo showcasing the initial primitives (Button + Overlay).
- `apps/web-demo`: adds MenuTrigger demo section (open/navigate/select).
- `apps/web-demo`: adds Select demo section (Button -> Popover -> ListBox).
- `apps/web-demo`: adds Checkbox/Switch demo section (Tab focus + Space toggle + focus-visible).
- `apps/web-demo`: Trunk entrypoint (`index.html`) and run instructions (`README.md`).
- `apps/tauri-demo`: Tauri v2 shell scaffold (config + build script + minimal command) for desktop verification.
- Dev tooling: `githooks/` (Conventional Commits + pre-commit gates) with `scripts/setup-githooks.sh`, plus gate runner scripts (`scripts/gate.sh`, `scripts/check.sh`) and `scripts/fetch_upstream.sh` for cloning upstream reference repos into `examples/` (ignored by git).
- Project docs: MVP/spec notes and a TODO/DAG-based implementation plan.
- Research: Android spike checklist and go/no-go criteria (`docs/research/android-spike.md`).

### Changed

- `ui-headless`: callback/handler types now use Leptos `Callback` (Send+Sync) to support rendering inside `Portal`.
- `ui-components`: re-exports `provide_focus_visible`, `provide_overlay_stack`, and `OnPress` to reduce app-layer coupling.
- `ui-core`: callback types are now `Send + Sync` (uses `Arc<dyn Fn(...) + Send + Sync>`).
- `ui-headless`: `use_press` now supports keyboard Enter/Space (with click de-duping) and exposes key handlers that indicate when callers should `preventDefault` (for custom elements).
- `ui-headless`: `use_button` now supports `ButtonElement` + returns `ButtonAttrs` (`role`/`tabindex`/`aria-disabled`) for custom button semantics.
- `ui-headless`: `use_listbox` now supports `on_action` to react to selection activation.
- Dev tooling: pre-commit now refuses oversized Rust files (default 1000 lines; override via `RUST_UI_MAX_RS_LINES=<N>`).
