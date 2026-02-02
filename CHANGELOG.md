# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

### Added

- Cargo workspace scaffold with layered crates (`ui-core`, `ui-headless`, `ui-theme`, `ui-components`) and demo apps (`web-demo`, `tauri-demo`).
- `ui-core`: initial headless state primitive `use_toggle_state` with unit tests.
- `ui-theme`: design tokens + CSS variable emitter, plus base/safe-area CSS helpers and unit tests.
- `ui-headless`: initial interaction primitives (focus-visible modality, press handling, button behavior) with `web`/`ssr` feature gating.
- `ui-components`: initial `<Button>` component integrating headless behavior + theme tokens.
- `ui-headless`: overlay primitives (topmost overlay stack + focus trap with focus restore).
- `ui-headless`: `use_focus_ring` hook for per-component focus ring handling.
- `ui-components`: initial `<Overlay>` (portal + Esc/topmost + click-outside + focus trap).
- `apps/web-demo`: minimal Leptos CSR demo showcasing the initial primitives (Button + Overlay).
- `apps/web-demo`: Trunk entrypoint (`index.html`) and run instructions (`README.md`).
- `apps/tauri-demo`: Tauri v2 shell scaffold (config + build script + minimal command) for desktop verification.
- Dev tooling: `githooks/` (Conventional Commits + pre-commit gates) with `scripts/setup-githooks.sh`, plus gate runner scripts (`scripts/gate.sh`, `scripts/check.sh`) and `scripts/fetch_upstream.sh` for cloning upstream reference repos into `examples/` (ignored by git).
- Project docs: MVP/spec notes and a TODO/DAG-based implementation plan.

### Changed

- `ui-headless`: callback/handler types now use Leptos `Callback` (Send+Sync) to support rendering inside `Portal`.
- `ui-components`: re-exports `provide_focus_visible`, `provide_overlay_stack`, and `OnPress` to reduce app-layer coupling.
