# FileTrigger

`FileTrigger` is a button-backed file input primitive: it forwards trigger press to an invisible `<input type="file">`.

## Goals / Non-goals / Risk Boundary

- Goal: provide accessible file picking with stable callback payload and motion contract reuse.
- Non-goal: no upload transport, validation pipeline, or app-specific file policy logic.
- Risk boundary: browser/file-input quirks stay encapsulated in `view.rs` and `logic.rs`.

## Architecture Layers

- `logic.rs`: consumes `ui-state-primitives::file_trigger` (`resolve_props` + `resolve_render_state`) and collects selected files into typed `FileTriggerFile`.
- `view.rs`: renders hidden input + trigger button, handles press/change bridging.
- `motion.rs`: `FileTriggerMotion` contract, sanitized via button motion sanitizer.
- `styles.rs`: static state selectors (`disabled`, `custom-motion`, etc.).
- `mod.rs`: exports `FileTrigger`, `FileTriggerFile`, `FileTriggerMotion`.

## File Responsibilities

- `mod.rs`: keeps minimal public boundary (`FileTrigger`, `FileTriggerFile`, `FileTriggerMotion`) without leaking implementation modules.
- `logic.rs`: normalization/state assembly + marker composition; no view rendering or style rule authoring.
- `styles.rs`: static token-first CSS only (`var(--ui-*)` markers), no runtime branching logic.
- `view.rs`: Leptos structure + headless semantic mounting + event wiring, with state derivation delegated to `logic.rs`.
- `motion.rs`: motion contract mapping/sanitization only, no custom animation engine implementation.

## Spec Policy

- `FileTrigger` does not include `spec.rs`; this is intentional because the component has no complex external schema/builder requirement.
- Contract notes stay in `check2.md` and this README. If `spec.rs` is introduced later, it must come with schema tests and version-evolution notes.

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `id` | `Option<String>` | `None` |
| `is_disabled` | `Option<bool>` | `None` (`false` after normalization) |
| `is_multiple` | `Option<bool>` | `None` (`false` after normalization) |
| `accept` | `Option<String>` | `None` |
| `is_accept_directory` | `Option<bool>` | `None` (`false` after normalization) |
| `capture` | `Option<String>` | `None` |
| `motion` | `FileTriggerMotion` | `FileTriggerMotion::default()` |
| `on_files` | `Option<Callback<Vec<FileTriggerFile>>>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `children` | `Children` | required |

Legacy compatibility:
- `disabled`, `multiple`, `accept_directory` are still accepted as aliases.
- `is_*` props win when both new and legacy aliases are provided.
- default fallback is centralized in `ui-state-primitives::file_trigger::resolve_props` with a single rule:
  `is_*` -> legacy alias -> `false`.
- mutually-exclusive file picking mode is typed in `ui-state-primitives` as
  `FileTriggerSelectionMode::{SingleFile, MultipleFiles, Directory}`.

Controlled/uncontrolled axis:
- N/A for this component. `FileTrigger` does not own a mutable value axis such as `value/default_value/on_value_change`.
- `is_disabled` / `is_multiple` / `is_accept_directory` are configuration inputs, and `on_files` is an event output only.

Async interaction:
- N/A for this component. It has no remote request and no async loading/error lifecycle.
- The component only delegates synchronous file-selection events and forwards files via `on_files`.

`FileTriggerFile` payload:
- `name: String`
- `size: u64`
- `mime: String`

## Hello World

```rust
<FileTrigger on_files=on_files>"Pick files"</FileTrigger>
```

## DX Paradox

- Default path keeps API minimal: users only provide `on_files`.
- Hello World stays within 1 line and runs directly.
- Advanced controls (`accept`, `is_multiple`, `is_disabled`, custom `motion`) are opt-in.
- No manual wiring of `ui-state-primitives` / `ui-headless` is required for baseline usage.
- docs-app exposes the same default path in `Quick Start (Default API)`.

## Composite API

- N/A for this component. `FileTrigger` is not a container that coordinates item collections.
- No `Parent/Item` paired API or `labels + children` / `titles + panels` parallel-array path is provided.

## Macro / Micro State Machine

- N/A for this component. `FileTrigger` does not expose drag gestures or frame-by-frame physics loops.
- No `Dragging` phase local loop or `Action::DragEnd` convergence channel exists in this component.

## Two-Pass Rendering

- N/A for this component. `FileTrigger` does not render overlay geometry (`Tooltip/Popover/Menu`) and does not perform DOM measure/rectification loops.
- There is no `Intent -> Measure(view) -> Rectification(logic)` pipeline in this component.

## Registration Protocol

- N/A for this component. `FileTrigger` is not a dynamic collection container (`Accordion/Tabs/Menu` style).
- No `RegistrationContext`, `Register/Unregister`, or `items_order` navigation contract exists in this component.

## Slot Projection

- N/A for this component. `FileTrigger` is not a container with projected content lifecycle (`Lazy/KeepAlive/Eager`).
- No `KeepAlive` hidden lifecycle or `NotifyHidden` side-effect throttling contract exists in this component.

## Env Streams

- N/A for this component. `FileTrigger` does not subscribe to `Resize/Theme/Intersection` environment streams.
- No debounce/throttle sampling or `BreakpointChanged`-like action fan-in pipeline is implemented for this component.

## Event Light Cone

- N/A for this component. `FileTrigger` is not a large collection surface (`Table/Grid`) with batch selection semantics.
- No `Context Bus + Selector` fan-out path or compressed state such as `SelectionState::All` exists in this component.

## Causality Bus

- N/A for this component. `FileTrigger` has no derived-command bus with broadcast subscribers.
- No `TraceId` passthrough chain (`user trigger -> derived command -> bus broadcast -> subscribers`) is required in this component.

## Focus Stack & GC

- N/A for this component. `FileTrigger` is not a layered `Overlay` and does not implement focus-restore stack behavior.
- The local `NodeRef` is used only for the hidden `<input type="file">` click bridge, not as an overlay focus recovery target.
- No `FallbackTo/Selector` focus-manager path or `document.body` fallback focus policy is implemented in this component.

## Escape Hatches

- N/A for this component. `FileTrigger` does not integrate imperative third-party runtime instances such as ECharts/Map.
- No `Foreign Zone` lifecycle (`YieldControl/CleanupForeign`) is required because there is no external imperative instance ownership boundary here.
- Public API does not expose third-party instance handles; component state remains isolated to primitive normalization + headless semantic mounting.

## Hydration Discontinuity

- This component does not generate runtime IDs internally and has no `now()`/random UUID path.
- `id` is caller-provided (`Option<String>`) and passed through directly to the hidden input node.
- Because no internal non-deterministic seed is used, SSR/Hydration ID stability is preserved without local `IdProvider` allocation in this component.

## SSR and Cross-Platform

- `view.rs` uses explicit compile-time branches for `wasm32` and `non-wasm` paths; platform differences are not hidden in runtime heuristics.
- `logic.rs` keeps `collect_files_from_input` available on both targets and the non-wasm branch avoids `web_sys` types.
- The non-wasm behavior is deterministic no-op file collection (`Vec::new()`), preserving SSR/tooling compile stability.

## ui-headless Feature Exclusivity

- `crates/ui-headless/src/lib.rs` enforces `web` vs `ssr` exclusivity through
  `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)`.
- `FileTrigger` only consumes headless semantic contracts (`use_file_trigger`) and does not introduce local feature-mixing logic.
- Compile-only checks for both paths are required by policy; in this environment they are currently blocked by `Invalid cross-device link (os error 18)`, so source-level guard plus semantic regression assertions are kept as contract evidence.

## ui-motion Non-wasm No-op

- `crates/ui-motion/src/lib.rs` provides non-wasm stubs (`web::prefers_reduced_motion() -> true`, `web::animate(...)` no-op) for SSR/tooling-safe compilation.
- `FileTrigger` does not own runtime animation handles; `motion.rs` only sanitizes and maps to shared button motion contract.
- Downstream `button::attach_motion` has explicit non-wasm fallback path and does not assume animation runtime availability.

## Reduced-motion / SSR / wasm Coverage

- Reduced-motion: animation attachment is delegated to shared `button::attach_motion`, which short-circuits when `ui_motion::web::prefers_reduced_motion()` is true.
- SSR/hydration: semantic contract mounting (`use_file_trigger`, `data-*`, `aria-*`) remains outside platform-only enhancement branches, keeping first-frame semantics stable.
- wasm enhancement: browser-only behaviors (`input.click`, directory/capture attribute mutation) are gated by `#[cfg(target_arch = "wasm32")]` and do not split semantic state contract from non-wasm/SSR output.

## Performance Budget

- Budget scope for this component:
  - first render: no polling loop / observer stream / per-frame animation loop introduced by `FileTrigger` itself
  - key update path: `on_press` and `on_change` remain direct O(1) control flow (excluding browser-provided file list traversal)
  - memory trend: no persistent interval/timer/task handles retained in component-local state
- Equivalent regression evidence (current component-level baseline):
  - source-guard test asserts absence of `requestAnimationFrame` / `setInterval` / `setTimeout` / async task fan-out primitives in `view.rs` + `logic.rs` + `motion.rs`
  - source-guard test constrains `view.rs` to a single `Effect::new(...)` usage for wasm-only attribute synchronization
- `render_count` precision instrumentation is tracked at repository-level test infrastructure; this component currently provides deterministic source-level baseline as fallback evidence.

## view! Macro Complexity

- `view.rs` keeps a single compact `view!` block for one semantic root (`span`) with two direct children (`input` + `Button`).
- No repeated deep template segments (`header/body/item` loops, repeated nested slots) are present in the component render tree.
- Regression guard asserts `view!` block count and structural markers to catch accidental macro-bloat drift that would affect compile time and wasm output size.

## Functional Decomposition

- `FileTrigger` keeps one public `#[component]` boundary (`FileTrigger`) and avoids fragment-level component proliferation.
- For future lightweight render fragments that do not need independent props semantics, prefer plain Rust helper functions returning `impl IntoView` over adding nested `#[component]`.
- Semantic markers and tests are anchored to the stable root structure, so decomposition changes must preserve test selector stability.

## Static Fragment Constantization

- Current `FileTrigger` render tree has no large static fragments (no complex SVG/footer/long static prose block), so no extra template constantization is required today.
- If future changes add heavy static fragments, move them to named constants/helper templates instead of embedding large static chunks directly in `view!`.
- Accessibility semantics (`title` / `aria-label` / `role`) must remain explicit and testable after constantization refactors.

## inner_html Safety

- `FileTrigger` does not use `inner_html` / `set_inner_html`.
- If a future change requires HTML injection, only trusted compile-time constants or explicit whitelist content may be used.
- User input, remote content, and dynamic template concatenation are prohibited for injected HTML paths and must be covered by dedicated semantic/security regression tests.

## WASM Debugability

- Component-level trace surface is semantic-first: `data-state`, `data-disabled`, `data-enabled`, `data-motion-source`, `data-custom-motion`, plus input `aria-hidden/tabindex`.
- Replay anchors are stable interaction hooks: trigger `on_press` -> hidden input `click` -> input `on:change` -> `on_files` callback payload.
- Visual debug entrypoints belong to docs-app/workbench layer; this component intentionally keeps debug-only feature flags and debug APIs out of its public surface to avoid production pollution.

## Docs Playground（展示区）

### 展示 (Display)

- workbench 展示 `FileTrigger` 实时文件选择行为和回调结果列表。
- 预览禁用态、多选态、自定义动效态。

### config

- `Accept`：`any / images / documents`。
- `Multiple`、`Disabled`、`Custom motion` 开关。

### code

- `code` 面板根据当前配置生成最小可复制片段。
- 动效开启时展示 `FileTriggerMotion` 结构体配置。

### css test

- `css test` 面板绑定 `components/file-trigger/src/styles.rs`。
- 可验证 `ui-file-trigger--disabled` 与 `ui-file-trigger--custom-motion` 的样式契约。

### 多场景对比显示

- `State Comparison` 同屏对比 default / disabled / custom motion 三种状态。

## Semantics and Accessibility

- Hidden input A11y (`tabindex`, `aria-hidden`) and locale passthrough (`lang`, `dir`) are mounted from the `ui-headless` contract.
- Root marker contract includes `data-state`, `data-disabled`, `data-enabled`, `data-motion-source`, `data-custom-motion`.
- Input value is cleared before click so selecting the same file again still emits `change`.
- User-visible trigger copy is provided by `children` (or app-level i18n), not hardcoded in `view.rs`.
- Shared semantic mapping comes from `ui_headless::use_file_trigger(FileTriggerOptions { state, lang, dir })`; the component does not re-implement A11y helpers.

## Observable Markers

- Stable marker surface for tests/automation: `data-state`, `data-disabled`, `data-enabled`, `data-motion-source`, `data-custom-motion`, plus input `aria-hidden` and `tabindex`.
- Closed-set values:
  - `data-state`: `ready | disabled`
  - `data-motion-source`: `default | custom`
  - `data-disabled` / `data-enabled` / `data-custom-motion`: present as `"true"` or omitted.
- Source-axis note: this component has no controlled/uncontrolled value axis; applicable source marker is motion source (`default` vs `custom`).

## Type System + Machine-Readable State

- Discrete mutual-exclusive input is typed as `FileTriggerSelectionMode::{SingleFile, MultipleFiles, Directory}` in `ui-state-primitives`, not free-form strings.
- Invalid combinations are normalized in one place by `resolve_render_state` (`logic.rs` only consumes/re-exports this contract).
- Machine-readable marker contract is stable and closed-set:
  - `data-state`: `ready | disabled`
  - `data-motion-source`: `default | custom`
  - `data-disabled` / `data-enabled` / `data-custom-motion`: `"true"` or absent
- Contract breakage is caught by compiler-visible types (`enum`/struct inputs) plus `components/file-trigger/test/semantics.rs` marker assertions.

## Style State Selectors

- `styles.rs` uses explicit stable selectors only: component class plus `data-*` state markers (`data-disabled`, `data-motion-source`, `data-custom-motion`).
- No fragile DOM-structure guessing (`:nth-child`, deep descendant chains) is used for state branches.
- Runtime view does not inject business inline styles; visual state switches are explained by semantic markers.

## Token-first Style Contract

- Component styles are defined in `components/file-trigger/src/styles.rs` as static CSS constants.
- Aggregation/injection path is feature-gated and centralized:
  - `crates/ui/src/css.rs` appends `crate::file_trigger::styles::CSS` under `component-file_trigger`
  - `crates/ui/src/root.rs` calls `crate::css::push_components_css(...)` when `inject_components_css=true`
- Spacing/color-related visual semantics consume `var(--ui-*)` tokens (e.g. `var(--ui-space-sm)`), with no parallel private token system.
- Utility-First and CSS-in-Rust are not used as the component default styling mechanism.

## Tree Shaking

- Package mode: `component-file_trigger` gates module exposure in `crates/ui/src/lib.rs`.
- Style gating: `crates/ui/src/css.rs` appends `file_trigger::styles::CSS` only when `component-file_trigger` (and `inject-css`) is enabled.
- Source mode: direct source-path module inclusion (`#[path = "../../../components/file-trigger/src/mod.rs"]`) keeps usage naturally component-scoped.
- Repository-level CI budget thresholds (artifact size caps) are governed outside this component folder.

## Visual Desire

- N/A for this single-component checklist: default-theme aesthetics, HeroUI-style visual benchmarking, and screenshot baselines are repository-level governance concerns.
- `FileTrigger` participates by consuming shared tokens and component CSS injection paths; it does not define global visual language or docs-wide screenshot baselines.

## Semantic Test Matrix

- Semantic-contract tests live in `components/file-trigger/test/semantics.rs` and assert `data-*` / `aria-*` markers instead of visual snapshots.
- Covered key paths:
  - state/source markers: `data-state`, `data-disabled`, `data-enabled`, `data-motion-source`, `data-custom-motion`
  - interaction wiring: trigger `on_press`, input `on:change`, disabled guards, same-file reselection reset
  - platform branches: explicit `#[cfg(target_arch = "wasm32")]` and non-wasm fallback path checks
- Matrix applicability notes:
  - controlled/uncontrolled axis is N/A for this component (event-output only, no mutable value axis)
  - keyboard/pointer semantics are delegated through `ui-headless` + `Button` contract and verified via mounted attrs/handlers
- Snapshot tests are optional supplements and are not used as contract authority in this component.

## Motion and Fallback

- Reuses `ButtonMotion` through `FileTriggerMotion { trigger }`.
- Motion values are sanitized before runtime usage.
- Non-wasm path remains compile-safe and deterministic.

## Source-first / Copy-Paste Ready

- Docs entry: `apps/docs-app/src/pages/components/pages/files.rs::file_trigger()`
- Source: `components/file-trigger/src/{mod,logic,view,motion,styles}.rs`
- Package mode feature: `component-file_trigger` (depends on `component-button` for trigger rendering; optional `inject-css`)

## DX Requirements

- Style edits go through static CSS (`styles.rs`) and docs-app `Playground` preview, so common styling feedback does not require a component-side wasm recompile loop.
- docs-app keeps interaction context (`accept/multiple/disabled/custom motion` toggles + selected-file list) during live edits.
- Workbench isolation canvas is provided by `apps/docs-app/src/pages/components/pages/files.rs` `Interactive Playground`.

## Engineering Baseline

- N/A for `serde`/spec migration in this component: `FileTrigger` has no external spec payload or versioned config schema input.
- N/A for runtime-agnostic async boundary in this component: there is no async API surface, and no `tokio`/`async-std` runtime type leaks through public props.
- Tracing policy stays repository-level; this component exposes stable semantic markers for correlation without local tracing protocol forks.

## Defensive Variables and Layering

- `styles.rs` uses token variables with fallback chains (`var(--ui-*, var(--ui-fallback-*))`) for spacing/opacity/sr-only sizing.
- Component CSS is aggregated into `@layer ui` via `crates/ui/src/css.rs`.
- Runtime style mutation is limited to semantic/custom-property channels; no raw inline layout styles are emitted from `view.rs`.

## Motion Contract

- `motion.rs` defines the component motion contract as `FileTriggerMotion { trigger: ButtonMotion }` and sanitizes inputs through shared button motion rules.
- Attachment and reduced-motion / non-wasm safety are delegated to `Button` + `ui-motion` no-op backend, so no local driver duplication exists in `FileTrigger`.

## Entry and File Layout

- `ui` integration remains feature-gated in `crates/ui/src/lib.rs` and `crates/ui/src/css.rs`; `root.rs` remains the unified injection/i18n root.
- Disallowed shared files (`overlay_open.rs`, `presence.rs`, `a11y.rs`) are not introduced in `crates/ui/src` by this component.
- Component directory keeps canonical files: `mod.rs`, `logic.rs`, `styles.rs`, `view.rs`, `motion.rs`; no `render.rs`.
- `spec.rs` is intentionally absent for this simple component surface.

## Manifest and RBI

- Context compression artifacts are now maintained at:
  - `components/file-trigger/src/Component.toml`
  - `components/file-trigger/src/file_trigger.rbi`
- These files project capabilities, semantic outputs, and public signature shape for tooling/agent indexing.

## Agent Contract Schema

- `logic.rs` defines typed agent contract enums/struct via `resolve_agent_contract(...)`.
- `view.rs` mounts schema markers:
  - `data-ui-schema`, `data-ui-schema-version`
  - `data-ui-intent`, `data-ui-action`, `data-ui-state`, `data-ui-source`
  - `data-ui-stream-support`, `data-ui-stream-fallback`, `data-ui-output-status`
- Contract fields are closed-set and derived from typed state, not ad-hoc string assembly in templates.

## Snapshot and Streaming Scope

- This checklist scope defines “streaming” only as LLM incremental-output rendering mode.
- `FileTrigger` is not a text-generation surface; it is `Streaming Optional` with explicit snapshot fallback (`data-ui-stream-support="optional"` + `data-ui-stream-fallback="snapshot"`).
- Snapshot rendering is always supported because the component consumes complete props and renders deterministically.

## Rust Hygiene

- Non-test source in `components/file-trigger/src/*` keeps `unwrap/expect` out of component logic, and avoids swallowed `let _ = ...`.
- String-class composition in `logic.rs` is implemented without hot-path string cloning patterns.
- Repository-wide hygiene command remains `./scripts/check-rust-hygiene.sh`; this component maintains local source guards for the same policy.

## Semantics / Performance / Delivery

- Semantic guard tests are source-of-truth in `components/file-trigger/test/semantics.rs` (`role/aria/data-*` and state-source markers).
- Render-count instrumentation remains repository-level; component-level fallback evidence is kept through deterministic source guards.
- No breaking API migration is introduced in this change set, so `migrate_v1_to_v2`/registry deprecation workflow is N/A.

## E2E / Docs Product

- E2E selector and repeatable-flow governance are handled at repository app/e2e layer; this component provides stable semantic selectors for that layer.
- docs-app page already includes:
  - Quick Start (Hello World)
  - Interactive Playground (config/state matrix)
  - Source-first code block with copy-ready imports
- Since `FileTrigger` API model did not change in this task, HeroUI parameter-strategy docs do not require sync updates in this patch.
