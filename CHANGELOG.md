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
- `apps/web-demo`: minimal Leptos CSR demo showcasing the initial primitives.
- Dev tooling: `scripts/check.sh` gate runner and `scripts/fetch_upstream.sh` for cloning upstream reference repos into `examples/` (ignored by git).
- Project docs: MVP/spec notes and a TODO/DAG-based implementation plan.
