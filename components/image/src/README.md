# Image

`Image` is a display component composed from `ui-state-primitives` + `ui-headless` + `ui-motion`.

## Goal / Non-goal / Risk Boundary

- Goal: provide a stable, accessible image primitive with fallback/skeleton/blur/zoom behavior.
- Non-goal: no business data fetching, no global state management, no theme orchestration in component layer.
- Risk boundary: if state/interaction rules drift across layers, fix them in primitives/headless/motion first, not in `view.rs` patches.

## Architecture Layers

- `logic.rs`: consumes `ui-state-primitives::image` contracts and centralizes props/default normalization (`normalize_props`).
- `view.rs`: Leptos structure render + locale attrs + semantic marker mount.
- `motion.rs`: `ImageMotion` contract, sanitize logic, wasm zoom spring driver, SSR no-op.
- `styles.rs`: static token-first CSS + reduced-motion fallback.
- `mod.rs`: minimal stable exports (`Image`, `ImageStatus`, `ImageRadius`, `ImageShadow`, motion helpers).

## File Responsibility Contract

- `mod.rs`:
  - keeps module boundary and minimal public export surface only.
- `logic.rs`:
  - owns input normalization, state derivation, and source marker typing.
  - does not perform DOM or CSS branching work.
- `styles.rs`:
  - keeps static CSS contract and token-variable consumption only.
- `view.rs`:
  - mounts Leptos structure and headless contracts; consumes normalized/derived outputs from `logic.rs`.
- `motion.rs`:
  - maps component semantic state to motion contract and attach lifecycle.
  - does not take over render tree ownership from `view.rs`.

## Spec.rs Policy Contract

- `Image` does not define `spec.rs`.
- Reason:
  - this component has no external stable schema contract or complex configuration freeze requirement.
  - adding `spec.rs` only for shape uniformity would be unnecessary abstraction for this component.
- If `spec.rs` is introduced in the future:
  - it must come with explicit schema/versioning rationale and matching contract tests.
  - until then, component docs and `check2.md` remain the source of component-level contract explanation.

## File Placement Discipline Contract

- Core component source layout is fixed:
  - `mod.rs`, `logic.rs`, `styles.rs`, `view.rs`, `motion.rs`.
- Repo-required protocol file remains present:
  - `protocol.rs` is kept as protocol/schema boundary and does not replace core file responsibilities.
- Forbidden/optional file policy:
  - `render.rs` is forbidden for this component.
  - `spec.rs` stays absent for current simple scope.

## Hyper-Structure Builder Contract

- `N/A` for `Image` in current scope:
  - `Image` is a display primitive without complex nested config graph requiring `*Spec::new()...render()` orchestration.
  - introducing builder-only `spec.rs` here would be abstraction noise, not value.
- Current contract:
  - keep `spec.rs` absent and keep protocol surface minimal/versioned in `protocol.rs`.
- Escalation rule:
  - if `Image` grows into a schema-heavy complex component, then add a typed builder entry (`ImageSpec::new()...render()`) with dedicated contract tests in the same change.

## Context Compression (Manifest + RBI) Contract

- `Image` keeps context-compression artifacts in component directory:
  - `components/image/src/Component.toml` stores capability manifest and typed input/output inventory.
  - `components/image/src/image.rbi` stores stable signature projection for tooling/agent indexing.
- Manifest/RBI drift policy:
  - when public props, semantic markers, or protocol boundary changes, `Component.toml` and `image.rbi` must be updated in the same change.

## Beginner-Friendly Path

- 如果你是第一次使用 `Image`，先走默认 API 路径（无需理解分层细节）：

```rust
<Image
  src=Some("https://example.com/photo.jpg".to_string())
  alt="Cover".to_string()
/>
```

- 文档顺序契约（先用起来，再进阶）：
  - 先看 `Hello World (Default API)`，只关注 `src + alt` 最小路径。
  - 再看常见场景矩阵（loaded/blurred/fallback/missing 与 controlled-N/A 对照）。
  - 最后进入 `Workbench` 与 API 表，按需启用 `is_blurred/is_zoomed/fallback_src/motion/radius/shadow` 等进阶参数。

## API (Table)

### Image Props

| Prop | Type | Default |
| --- | --- | --- |
| `src` | `Option<String>` | `None` |
| `alt` | `String` | required |
| `fallback_src` | `Option<String>` | `None` |
| `is_skeleton_disabled` | `bool` | `false` |
| `is_blurred` | `bool` | `false` |
| `is_zoomed` | `bool` | `false` |
| `radius` | `ImageRadius` (`Sm` / `Md` / `Lg` / `Full`) | `Lg` |
| `shadow` | `ImageShadow` (`None` / `Sm` / `Md`) | `Sm` |
| `motion` | `ImageMotion` | `ImageMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | component does not expose explicit callbacks | `-` |

### Naming Migration

- `disable_skeleton` has been renamed to `is_skeleton_disabled` to align with the `is_*` boolean API contract.

### Controlled / Uncontrolled

- `N/A`: `Image` does not expose a controllable state axis (`value/on_*_change/default_*`); current boolean props are stateless render configuration inputs.

### Composition Contract

- `N/A`: `Image` is not a composite container component and does not expose `Parent/Item` composition API.
- Disallowed API shapes: `labels + children`, `titles + panels`, or `ItemSpec`-style parallel list contracts.

## Hello World

```rust
<Image
  src=Some("https://example.com/photo.jpg".to_string())
  alt="Cover".to_string()
/>
```

- DX baseline:
  - Minimal path is `src + alt` only (no manual state/headless wiring).
  - Hello World stays within 5 lines.
  - Advanced behavior is opt-in via optional props (`is_blurred`/`is_zoomed`/`fallback_src`/`motion`).

## Docs Playground 展示区

`apps/docs-app` 中的 `Image` 页面已补齐文档产品化入口：

- `Hello World (Default API)`：最小可运行调用路径。
- `State Matrix: Loaded / Blurred / Fallback / Missing`：状态矩阵覆盖关键渲染分支。
- `Controlled vs Uncontrolled (N/A)`：显式说明该轴在 `Image` 上为 N/A，并展示“默认调用 vs 上游映射”对照。
- `Streaming Optional / Snapshot`：展示 `streaming optional + fallback=snapshot` 的阅读模式边界。
- `Source-first Starter (Copy-Paste Ready)`：一键复制 starter 代码。
- `Workbench: Display + Config + Code + CSS Test`：继续作为交互式调试与配置快照入口。

`Playground` 的复制路径统一经 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 补全缺失 imports，避免“复制即报缺依赖”。

## Documentation as Product Contract

- Docs-app 页面必须满足 Copy-Paste Ready：
  - playground snippet 复制后可直接运行（imports 自动补齐）。
  - Source-first 区块给出依赖前置（`component-image` + `UiRoot/inject-css`）。
- 状态矩阵/对照矩阵必须在 docs 层显式可见：
  - 状态矩阵覆盖 loaded/blurred/fallback/missing。
  - 受控/非受控轴对 `Image` 明确标记为 `N/A`，不做伪受控 API 展示。
- 流式策略必须文档化：
  - `Image` 非正文阅读面，采用 `Streaming Optional / Snapshot`。

## Interactive Playground Contract

- `apps/docs-app` 的 Image Workbench 提供在线可调 props/state：
  - segmented controls: `source` / `radius` / `shadow` / `motion`
  - switches: `is_zoomed` / `is_blurred` / `is_skeleton_disabled` / `with_fallback` / `custom_class`
- 同一画布实时预览：
  - `data-slot="image-workbench-stage"` 渲染实时 `Image` 输出，并显示状态摘要（source/fallback/zoomed/blurred）。
  - `ImageActualConfig` 通过 `test_config_signal=actual_config` 暴露当前输入快照，便于复现与回放。
- AI Spec 子项适用性：
  - `Image` 不是 AI Spec/schema 驱动组件，本条中的 Spec 输入联动按职责为 `N/A`。
  - 不引入伪 `spec.rs`，改用类型化配置快照保证“输入 -> 预览”联动可验证。
- 可重复验收路径：
  - `e2e/tests/docs_app_image_contract.spec.mjs` 覆盖 workbench 关键流（切换 source -> 断言语义标记 -> reload 复验）。

## DX Contract

- Workbench isolation entry:
  - docs-app exposes `Playground title="Workbench: Display + Config + Code + CSS Test"` for Image-specific isolated rehearsal.
- Style-debug fast path:
  - workbench binds `test_css_source` and `test_source_path="components/image/src/styles.rs"`, so CSS-contract verification is anchored to `styles.rs`.
- Context continuity:
  - controls and preview share the same `signal` state axes (`source/radius/shadow/motion/zoomed/blurred/skeleton/fallback/custom_class`), keeping interaction context across edits.
- Optional state snapshot:
  - workbench emits `ImageActualConfig` as current-state snapshot for reproducible debugging.
- Visual observation anchor:
  - docs page keeps `data-visual-baseline="image-default-theme"` as a stable observation hook during development and regression checks.

## Semantics and Accessibility

- `alt` is required by API.
- Locale plumbing is mounted via `lang` / `dir` on root wrapper.
- Stable semantic markers are exposed on root:
  - `data-slot="image-wrapper"`
  - `data-state` (`idle` / `loading` / `loaded` / `error`)
  - `data-status-source` (`initial` / `event`)
  - `data-loaded`, `data-zoomed`, `data-fallback`, `data-skeleton`, `data-blurred`
  - `data-radius`, `data-shadow`
  - `data-motion-source`, `data-custom-motion`
- Stable slots are exposed for style/test contracts:
  - `data-slot="image"`
  - `data-slot="image-fallback"`
  - `data-slot="image-skeleton"`
  - `data-slot="image-blurred"`

## Motion and Fallback

- `ImageMotion` defaults:
  - `zoom_spring = ui_motion::presets::spring_soft()`
  - `zoom_scale = 1.03`
- Custom motion values are sanitized (`finite`, positive spring params, zoom range clamped to `[1.0, 4.0]`).
- wasm path writes `--ui-image-zoom` with spring animator.
- non-wasm path is no-op and still sanitizes config to keep SSR/tooling builds deterministic.
- reduced-motion disables skeleton shimmer and zoom transform effects.

## Motion Contractization

- Motion contract is component-local and typed in `motion.rs`:
  - `ImageMotion { zoom_spring, zoom_scale }` defines the explicit tunable contract.
  - `sanitize_spring` enforces valid positive `stiffness/damping/mass/precision` and falls back to default spring preset.
- Attach boundary is explicit:
  - `view.rs` maps semantic state to `attach_zoom_motion(...)` only.
  - motion execution details remain in `motion.rs`, not in render logic.
- Reduced-motion and platform degradation:
  - wasm attach path checks `ui_motion::web::prefers_reduced_motion()`; when true, it resets `--ui-image-zoom` to `1` and skips animator attach.
  - non-wasm/SSR path is deterministic no-op (`black_box(sanitize_motion(motion))`) and never touches browser APIs.

## Macro / Micro Interaction Contract

- `N/A` for drag macro/micro dual-state machine:
  - `Image` has no drag/pan gesture model and therefore no `Dragging`/`DragEnd` action pair.
  - High-frequency updates are limited to local hover zoom animation in `motion.rs` (`SpringAnimator` target updates), not per-frame state-machine writes into `logic.rs`.

## Two-Pass Geometry Rendering Contract

- `N/A` for geometry two-pass rendering (`Intent -> Measure(view) -> Rectification(logic)`):
  - `Image` is not a tooltip/popover/menu-style overlay and does not depend on DOM rect measurement for placement.
  - `view.rs` only mounts static image/fallback/skeleton structure and status semantics; no geometry measurement loop exists.
  - `logic.rs` has no rectification reducer for measured layout deltas, so there is no convergence loop to guard in this component.

## Registration Protocol Contract

- `N/A` for collection registration protocol (`RegistrationContext` + `Register/Unregister` + `items_order`):
  - `Image` is not a collection container (`Accordion/Tabs/Menu`) and has no dynamic child item registry.
  - `logic.rs` does not maintain item order/navigation state and does not consume `HashSet` iteration for focus/nav behavior.
  - component API has no `Item`/`items` axis, so registration lifecycle contracts are out of scope.

## Slot Projection Contract

- `N/A` for container slot projection policy (`Lazy/KeepAlive/Eager`):
  - `Image` renders a fixed local structure (`image`/`fallback`/`skeleton`) rather than projecting dynamic container child slots.
  - component has no keep-alive hidden lifecycle or `NotifyHidden` callback path.
  - no polling/animation side effect depends on projected slot visibility state.

## Env Streams Contract

- `N/A` for environment stream pipeline (`Resize/Theme/Intersection -> sampled/debounced -> Action -> logic`):
  - `Image` does not subscribe to resize/theme/intersection sources to derive semantic state.
  - `view.rs` has no env-event sampling/debounce path, and `logic.rs` has no `BreakpointChanged`-style action reducer.
  - component state only reacts to local image resource events (`load/error`) and explicit props.

## Event Light Cone Contract

- `N/A` for large-collection event light-cone pipeline (`Context Bus + Selector + compressed state`):
  - `Image` is a single display primitive, not a `Table/Grid`-style bulk interaction surface.
  - component has no `SelectionState::All`-style compressed selection model.
  - no O(N) descendant prop-drilling path exists for batch operations.

## Causality Bus Contract

- `N/A` for causality bus trace pipeline (`user trigger -> derived command -> bus broadcast -> subscriber`, `TraceId` passthrough):
  - `Image` has no cross-module command bus or multi-subscriber derivation graph.
  - `logic.rs` only reduces local status transitions and does not carry trace context metadata.
  - component behavior is local and direct, so there is no broken causal chain risk to reconcile.

## Focus Stack & GC Contract

- `N/A` for overlay focus stack / restore protocol:
  - `Image` is not an overlay/layered dialog/popover container and does not own focus-trap lifecycle.
  - component does not keep focus-restore target refs or private focus stack state.
  - local `NodeRef<html::Div>` usage exists only for motion attach and does not participate in focus restoration.

## Escape Hatch / Foreign Zone Contract

- `N/A` for command-style third-party integration boundary:
  - `Image` does not host imperative third-party runtimes (ECharts/Map-like instances).
  - component API does not expose foreign instance handles or cleanup control surface.
  - no `YieldControl/CleanupForeign` workflow is required in this component scope.

## Hydration Discontinuity Contract

- `N/A` for component-local deterministic id seeding:
  - `Image` does not allocate local ids and does not initialize logic from `now()`/random UUID sources.
  - `view.rs` / `logic.rs` / `motion.rs` remain deterministic from props + resource events only.
- Deterministic seed boundary is owned by root assembly:
  - `UiRoot` exposes `id_seed` and calls `provide_ui_id_provider(id_seed)` in `crates/ui/src/root.rs`.
  - components that need generated ids consume this root-provided provider instead of local randomness.

## SSR / Cross-Platform Compile Contract

- Compile-only verification commands (web/ssr/native):
  - `cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-image,inject-css`
  - `cargo check -p ui-headless --no-default-features --features ssr`
  - `cargo check -p ui`
- Platform split is explicit in `components/image/src/motion.rs`:
  - wasm implementation uses `#[cfg(target_arch = "wasm32")]` for DOM-backed zoom spring attach.
  - non-wasm implementation uses `#[cfg(not(target_arch = "wasm32"))]` and keeps deterministic no-op semantics.
- Non-wasm safety rule:
  - browser-only APIs (`web_sys`/DOM globals) are confined to wasm branch and must not appear in non-wasm branch body.

## Ui-Headless Web/SSR Feature Mutex Contract

- Mutual-exclusion guard lives in `crates/ui-headless/src/lib.rs`:
  - `#[cfg(all(feature = "web", feature = "ssr"))]`
  - `compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one")`
- Compile-only verification commands:
  - `cargo check -p ui-headless --no-default-features --features web`
  - `cargo check -p ui-headless --no-default-features --features ssr`
  - `cargo check -p ui-headless --no-default-features --features web,ssr` (must fail via `compile_error!`)
- Component integration boundary:
  - `Image` consumes `ui-headless` contracts (`locale_attrs`, `use_hover`) and does not re-implement or bypass headless feature gating in component layer.

## Ui-Motion Non-Wasm No-op Contract

- Non-wasm stub backend is defined in `crates/ui-motion/src/lib.rs`:
  - `#[cfg(not(target_arch = "wasm32"))] pub mod web`
  - `prefers_reduced_motion() -> true`
  - `animate(...) {}` no-op implementation.
- Component motion degrade rule:
  - `components/image/src/motion.rs` keeps `#[cfg(not(target_arch = "wasm32"))] attach_zoom_motion(...)` as deterministic no-op and does not assume animator handle availability.
  - current path only sanitizes motion input (`std::hint::black_box(sanitize_motion(motion))`) and avoids panic/DOM calls.
- Compile-only verification command:
  - `cargo check -p ui-motion`

## Reduced-Motion / SSR / Wasm Branch Contract

- Reduced-motion fallback:
  - `components/image/src/styles.rs` defines `@media (prefers-reduced-motion: reduce)` to disable skeleton shimmer animation and zoom transform feedback.
- SSR/hydration semantic stability:
  - `components/image/src/view.rs` keeps semantic markers (`data-state`, `data-status-source`, `data-motion-source`) in the shared render path.
  - semantic state derivation stays in `logic.rs`; no target-specific semantic branch in `view.rs`.
- Wasm enhancement boundary:
  - `components/image/src/motion.rs` uses wasm-only spring driver for zoom animation.
  - non-wasm branch remains sanitize-only no-op, so SSR/tooling builds do not depend on runtime animation handles.
- Contract rule:
  - wasm branch may enhance interaction smoothness, but it must not change semantic marker protocol relative to SSR/non-wasm output.

## Performance Governance Contract

- Image budget baseline:
  - first render budget: a single derived render-state memo (`view_state`) from normalized inputs.
  - critical update budget: resource events (`on:load` / `on:error`) perform one status transition via `logic::apply_status_event`.
  - memory trend budget: no growing runtime registry in `logic.rs`; wasm animator has cleanup stop; non-wasm path is sanitize-only no-op.
- Regression detection strategy:
  - `Image` is not a high-frequency/heavy interaction component, so strict per-component `render_count` budget is treated as `N/A` here.
  - current harness does not provide precise `render_count` automation for this component.
  - equivalent contract evidence is enforced through semantic tests on state/render/style/motion path anchors.
  - follow-up work should align with repository-level automated `render_count` gate used by base components.
- Attribution boundary:
  - state path: `logic.rs` normalization + status reducer.
  - render path: `view.rs` memo derivation + semantic marker output.
  - style path: `styles.rs` token-first static rules.
  - motion path: `motion.rs` wasm spring branch and non-wasm deterministic no-op branch.
- Repository baseline note:
  - `Button`/`Input` render-count budget (`=1` post-init without interaction) remains a workspace-level gate and is not redefined by Image.

## View Macro Complexity Contract

- Macro size boundary:
  - `components/image/src/view.rs` keeps one top-level `view!` block as render shell.
  - current conditional slots are shallow and explicit (`blurred` / `fallback` / `image` / `skeleton`) to avoid deep nested macro trees.
- Growth control:
  - if conditional branches increase or repeated nested fragments appear, extract local render helpers before expanding one giant `view!` body.
  - prioritize semantic sub-block slicing over accumulating nested inline markup.
- Regression guard:
  - semantic tests enforce a complexity ceiling on `view.rs` (macro count, `Show` branch count, source line budget) as an early signal for compile-time/wasm-size regression risk.

## Functional Decomposition Contract

- Keep one public component and move light fragments to plain functions:
  - `#[component]` is only used by `Image`.
  - lightweight render fragments are plain functions returning `impl IntoView`:
    - `render_blurred_layer`
    - `render_fallback_layer`
    - `render_image_layer`
    - `render_skeleton_layer`
- Decomposition boundary:
  - helpers stay in `view.rs` and only render stable semantic slots/events.
  - no helper is promoted to `#[component]` unless it needs an independent public props contract.
- Regression guard:
  - semantic tests verify helper presence, enforce single-`#[component]` count in `view.rs`, and ensure checklist/docs stay synchronized.

## Static Fragment Constantization Contract

- Constantized static fragments in `view.rs`:
  - static classes: `BLURRED_CLASS`, `FALLBACK_CLASS`, `IMAGE_CLASS`, `SKELETON_CLASS`
  - decorative-alt contract: `DECORATIVE_ALT_TEXT`
- Boundary:
  - only pure static literals are centralized; semantic state/data markers remain explicit in render tree.
  - this component has no heavy static SVG/footer/long text template requiring `inner_html` or external template files.
- A11y preservation:
  - constantization does not remove `aria-hidden="true"` on decorative layers.
  - `data-slot="image-*"` anchors remain stable for tests and automation selectors.

## Inner HTML Safety Contract

- `N/A` by design for this component:
  - `Image` has no `inner_html` render path and does not render raw HTML payloads.
  - all rendered output is explicit typed nodes/attributes in `view.rs`.
- Safety boundary:
  - disallow `inner_html`, `set_inner_html`, and `dangerously_set_inner_html` usage in component source.
  - user input and remote payloads must not be interpreted as HTML markup in component render path.
- Regression guard:
  - semantic tests assert absence of inner-html injection APIs across `view.rs` / `logic.rs` / `motion.rs`.

## WASM Debug Contract

- Traceability anchors (source / before / after):
  - source axis: `data-status-source` (`initial|event`) from `logic::ImageStatusSource`.
  - state axis: `data-state` (`idle|loading|loaded|error`) from primitive/view-state derivation.
  - transition reduction is explicit in `logic::apply_status_event`, so state-before/state-after path is deterministic and auditable.
- Minimal replay chain:
  - key event path is fixed: `on:load` / `on:error` -> `ImageStatusEvent` -> `logic::apply_status_event`.
  - docs-app debug/review flow can replay by ordered event sequence without requiring ad-hoc runtime hooks in component API.
- Visual debug entry:
  - docs-app `Image` page keeps stable baseline marker `data-visual-baseline="image-default-theme"` for development observation and regression tooling.
- Feature isolation:
  - `components/image/Cargo.toml` declares `wasm-debug` feature and keeps it opt-in (`default = []`).
  - component public API does not expose `is_debug`/`debug_*` props; debug capability stays internal and opt-in.

## A11y / I18n / L10n Contract

- `Image` keeps baseline a11y/i18n entry points:
  - `alt` is required and passed to visible image/fallback rendering.
  - `lang` / `dir` are consumed through `ui_headless::locale_attrs` and mounted on the root wrapper.
  - decorative layers (`blurred` preview and skeleton) are marked `aria-hidden="true"`.
- User-visible text is not hardcoded in `view.rs`; text comes from inputs (`alt`) or upper-layer docs/app composition.
- Non-interactive scope statement:
  - `Image` does not expose keyboard-triggered control semantics (`role=button`, key handlers, focus management), so interactive-role/keyboard path requirements are not applicable for this component shape.
- Component does not define local `aria_*` helper utilities; shared semantic utilities remain in `ui-headless`.

## State Observability Contract

- Stable machine-readable markers are mounted for state and source:
  - state axis: `data-state`, `data-loaded`, `data-fallback`, `data-skeleton`, `data-blurred`, `data-radius`, `data-shadow`.
  - source axis: `data-status-source` (`initial|event`) and `data-motion-source` (`default|custom`).
- Marker value sets are closed enums to prevent contract drift:
  - `data-state`: `idle|loading|loaded|error` (from primitive `status_attr`).
  - `data-status-source`: `initial|event`.
  - `data-motion-source`: `default|custom`.
- Selector guidance:
  - tests and automation should prefer `data-*` / `aria-*` contracts instead of DOM depth/order assumptions.
- Controlled/uncontrolled source marker is not applicable for `Image` because it has no controllable state axis.

## Style Explicit-State Contract

- `styles.rs` state branches rely on explicit semantic markers and stable classes:
  - marker selectors: `data-state`, `data-loaded`, `data-radius`, `data-shadow`, `data-custom-motion`.
  - stable structure classes: `.ui-image__img`, `.ui-image__fallback`, `.ui-image__blurred`, `.ui-image__skeleton`.
- Disallowed selector shapes:
  - no `:nth-child` / `:nth-of-type` / DOM-depth guessing selectors for state decisions.
- Runtime styling boundary:
  - `view.rs` does not emit business inline style attributes.
  - motion runtime only updates CSS custom property `--ui-image-zoom`.
- Visual state transitions stay explainable by semantic markers:
  - skeleton hide/show is driven by `data-state="loaded"` / `data-loaded="true"` marker contracts.

## Token-First Static Style Contract

- Static style source and injection path:
  - component style rules live in `components/image/src/styles.rs`.
  - component CSS is aggregated in `crates/ui/src/css.rs` behind `component-image` feature.
  - global injection is centralized in `UiRoot` via `crate::css::push_components_css`.
- Token-first value policy:
  - visual primitives (background/border/radius/shadow) consume `var(--ui-*)` variables.
  - component does not introduce parallel private token namespace outside `--ui-*`.
- Runtime style mutation boundary:
  - `view.rs` does not emit business inline styles.
  - runtime motion only writes CSS custom property `--ui-image-zoom`.
- Styling paradigm boundary:
  - Utility-First classes are allowed for `apps/*` composition only, not as component-library default contract.
  - CSS-in-Rust is not used in this component and remains an exception path requiring explicit net-benefit justification.

## Defensive Variables Contract

- `styles.rs` consumes defensive two-hop variable chains:
  - representative paths include `var(--ui-bg, var(--ui-fallback-bg))`, `var(--ui-border, var(--ui-fallback-border))`, `var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))`, `var(--ui-image-blur, var(--ui-fallback-image-blur))`.
- Component stylesheet does not keep hardcoded hex or bare terminal size constants for key visual semantics.
- Fallback terminal values are centralized in `crates/ui-theme/src/css.rs`:
  - image-specific fallbacks (`--ui-fallback-image-*`) and shared fallback (`--ui-fallback-radius-full`) are emitted by theme layer and consumed by component style contract.
- SSOT boundary:
  - component layer defines selector/state contracts only;
  - theme layer owns fallback terminal values and token defaults.

## Cascade Layer Contract

- CSS aggregation layer:
  - component stylesheet is aggregated by `crates/ui/src/css.rs::push_components_css`.
  - aggregation is wrapped by `@layer ui` and includes image CSS behind `component-image` feature gate.
- Runtime style mutation boundary:
  - `view.rs` does not emit business inline styles (`style="..."` / `style:top=...`).
  - runtime motion path only mutates CSS Custom Property (`--ui-image-zoom`) and does not write layout properties (`top/left/width/height`).
- Contract intent:
  - keep cascade ordering deterministic through layer-based aggregation.
  - keep runtime visual adjustments constrained to custom properties, not ad-hoc inline layout styles.

## Visual Desire Contract

- Baseline entry in docs-app:
  - `apps/docs-app/src/pages/components/pages/display.rs` exposes `Playground title="Default Theme Visual Baseline (Visual Desire)"`.
  - baseline root uses stable marker `data-visual-baseline="image-default-theme"` for screenshot/regression tooling hooks.
- Aesthetic acceptance scope for this component:
  - verify default theme hierarchy/contrast/feedback through the baseline block plus `Comparison Matrix` in the `Image` page.
  - visual language target aligns with HeroUI-level modern quality, not API-shape cloning.
- Scope boundary:
  - cross-component screenshot baselines for `Button/Input/Overlay` are repository-level gates.
  - `Image` documents and exposes its own baseline hook; it does not duplicate global visual-regression orchestration logic.

## HeroUI Benchmark Sync Contract

- Strategy-doc synchronization:
  - `docs/spec/heroui-parameter-design-strategy.md` keeps `### Image 同步记录（2026-02-20）`.
  - Parameter-semantic changes for `Image` must be synced there first, before checklist closure.
- Component-doc entry accessibility:
  - docs-app indexed entry: `apps/docs-app/src/pages/components/pages.rs` via `component_doc!("Image", "image", "Display", display::image)`.
  - docs page entry: `apps/docs-app/src/pages/components/pages/display.rs` with `slug="image"`.
  - equivalent component entry: `components/image/src/README.md`.
- Research-doc scope:
  - this Image sync does not introduce new Spectrum/HeroUI style conclusions, so `docs/research/spectrum-heroui-style-interface-study.md` remains unchanged.
- Merge policy:
  - interface-change scenarios fail review if they contain only code updates without synchronized docs updates.

## Tree Shaking Contract

- Package-mode feature slicing:
  - `crates/ui/Cargo.toml` keeps `component-image = ["dep:ui-image"]`.
  - `ui-image` dependency stays `optional = true`, so image code path is not mandatory in minimal feature builds.
- Export and CSS aggregation gates:
  - `crates/ui/src/lib.rs` exports `image` behind `#[cfg(feature = "component-image")]`.
  - `crates/ui/src/css.rs` injects image CSS behind `#[cfg(feature = "component-image")]`.
  - CSS aggregation itself is behind `inject-css`; non-inject path is explicit no-op.
- Source-mode boundary:
  - source consumers can depend on `components/image` directly without requiring an always-reachable central registry.
- Verification commands used:
  - `cargo tree -e features -p ui --no-default-features --features component-image,inject-css`
  - `cargo tree -e features -i ui -p web-demo`
- Current verification snapshot:
  - minimal feature tree shows `component-image` chain without `all-components`.
  - reverse dependency tree for `web-demo` is driven by `web-demo-components`, not hidden all-component enablement.

## Ui-Components Fixed Entry Contract

- `crates/ui/src/lib.rs` remains the public export boundary:
  - `image` export is behind `#[cfg(feature = "component-image")]`.
  - no direct `web-sys`/DOM details are re-exported through the image public API.
- `crates/ui/src/css.rs` remains the component-css aggregation entry:
  - `push_components_css` is the fixed aggregation function.
  - image stylesheet injection is feature-gated (`component-image`) and not unconditional.
- `crates/ui/src/root.rs` remains centralized injection + i18n entry:
  - `UiRoot` provides `UiI18n` + deterministic id provider.
  - base css + theme css vars + optional component css are composed in one place.
- `crates/ui-visual-primitive/src/active_highlight.rs` remains shared visual primitive:
  - file contains generic highlight style/motion contract (`ActiveHighlightMotion + attach_active_highlight_motion`).
  - no image-specific business semantics are embedded.
- Forbidden file-path checks stay enforced:
  - `crates/ui/src/overlay_open.rs` does not exist.
  - `crates/ui/src/presence.rs` does not exist.
  - `crates/ui/src/a11y.rs` does not exist.
- Headless ownership stays canonical:
  - open-state primitive remains in `crates/ui-headless/src/controllable_state.rs`.
  - presence primitive remains in `crates/ui-headless/src/presence.rs`.
  - shared a11y helpers remain in `crates/ui-headless/src/a11y.rs`.

## Semantic Testing Contract

- `Image` uses semantic-contract assertions, not visual snapshots, as primary regression gate.
- Coverage matrix (with applicability notes):
  - controlled/uncontrolled axis: covered as `N/A` (display component, no controllable state machine axis).
  - disabled axis: covered as `N/A` (no disabled interaction protocol surface).
  - keyboard path: covered as `N/A` (non-interactive image primitive; no key handler/role control contract).
  - focus-flow path: covered as `N/A` through dedicated focus-stack non-applicability contract (image is not an overlay/focus-trap component).
  - pointer path: covered by hover contract mount assertions (`on:pointerenter` / `on:pointerleave`, `use_hover`).
  - SSR/wasm split: covered by motion contract assertions (`#[cfg(target_arch = \"wasm32\")]` + non-wasm no-op path).
- Test locations:
  - component-local: `components/image/test/semantics.rs`.
  - workspace contract: `components/image/test/image/semantics.rs`.
- Snapshot policy:
  - no snapshot assertions are used as the primary acceptance signal for this component contract.

## Repeatable E2E Key Flow Contract

- E2E regression entry is fixed at `e2e/tests/docs_app_image_contract.spec.mjs`.
- Repeatable key flow:
  - `gotoImageDocsAndWaitSettled` waits for semantic readiness (`body:not(:has(#boot))` + `data-slot="image-wrapper"[data-state]`).
  - key interaction path uses semantic selectors only: open workbench settings -> switch source mode -> assert semantic state/source markers.
  - reload is included and semantic markers are re-asserted to ensure repeatability across navigation cycles.
- Breakpoint localization boundary:
  - failures are expected to point to concrete semantic markers (`data-slot`, `data-state`, `data-fallback`, `data-status-source`) instead of generic screenshot/page mismatch.
- High-risk applicability:
  - async path is in-scope and covered through ready/settled + status-source assertions.
  - overlay/focus/keyboard are `N/A` for this non-overlay, non-keyboard-driven display component.

## Async Contract

- `N/A` for application-level async action protocol:
  - `Image` does not initiate async actions (`use_async_action`/fetch/mutation) and does not expose `is_loading`/`retry`/`disabled`/`aria-busy` API.
  - Component only reacts to browser resource events (`on:load` / `on:error`) and maps them to `ImageStatus` semantics.

## Engineering Capability Contract

- Serde / spec/config boundary:
  - `components/image/src/protocol.rs` defines versioned protocol structs (`ImageComponentSchemaVersion` / `ImageComponentSpec`) with `serde` derive and explicit `schema_version`.
  - `Image` public API itself does not expose spec/config payload input, so no component-local migration parser or runtime schema switch is required in `view.rs`.
- Tracing boundary:
  - this component currently has no cross-module async workflow requiring local tracing spans/events; state reduction remains local (`on:load` / `on:error` -> `logic::apply_status_event`).
  - if tracing is added later, it should follow workspace tracing vocabulary and stay out of public API surface.
- Async runtime leakage boundary:
  - component source does not bind to tokio/async-std runtime types and does not expose runtime handles in public props/exports.
  - runtime details stay internal to platform event wiring; external API remains runtime-agnostic typed props.

## Version Deprecation Migration Contract

- `N/A` for current `Image` change set:
  - no cross-major breaking API upgrade is introduced.
  - `components/image/src/protocol.rs` stays on `ImageComponentSchemaVersion::V1`.
  - `components/image/src/Component.toml` keeps `schema_version = "1"`.
  - agent contract id stays `ui.image.agent-contract/v1`.
- Migration trigger boundary:
  - if future breaking upgrade introduces `V2` or removes/renames public semantic contract fields, Schema Registry must register a deprecation window and a pure migration function `migrate_v1_to_v2` must be added.
  - current component scope intentionally keeps `migrate_v1_to_v2` absent to avoid fake migration complexity when no breaking upgrade exists.

## State Primitive Contract

State contracts are centralized in `crates/ui-state-primitives/src/image.rs`:

- `ImageStatus`
- `ImageRadius`
- `ImageShadow`
- `ImageViewState`
- `resolve_view_state(...)`

Component layer only maps these outputs into render semantics.
All component-level fallback/default priority (`src`/`fallback_src`/`class_name`/`lang`) is normalized in `components/image/src/logic.rs`.
All render-state inputs are type-bound by `ImageViewStateInput` in `components/image/src/logic.rs` before deriving view semantics.
Mutually-exclusive discrete axes (`ImageStatus`/`ImageRadius`/`ImageShadow`/`ImageMotionSource`) are enum-constrained; `is_zoomed`/`is_blurred`/`is_skeleton_disabled` remain independent boolean toggles, not a shared state machine.
State primitive source stays in `ui-state-primitives`; `components/image/src/logic.rs` only maps/assembles primitive outputs and does not bind business/global store types.

## Type + Semantic Machine-Readable Contract

- Discrete state axes are type-constrained:
  - primitives: `ImageStatus`, `ImageRadius`, `ImageShadow`.
  - component source axes: `ImageMotionSource`, `ImageStatusSource`.
- Invalid combinations are normalized at logic boundary:
  - `logic::normalize_props` and `logic::derive_view_state` are the single normalization/derivation entrypoints.
- Stable machine-readable markers are mandatory outputs:
  - `data-state`, `data-status-source`, `data-motion-source`, `data-radius`, `data-shadow`.
- Contract breakpoints stay compiler/test-visible:
  - type drift is caught by enum-based signatures.
  - semantic drift is caught by component-local and workspace semantic tests.

## Agent Contract / Snapshot

- Agent Contract schema markers are explicit and typed:
  - `data-ui-schema` uses `protocol::IMAGE_AGENT_SCHEMA`.
  - `data-ui-intent` / `data-ui-action` / `data-ui-state` / `data-ui-*-source` / `data-ui-stream-*` / `data-ui-llm-mode` / `data-ui-output-status` all come from typed enums or typed mapping helpers in `protocol.rs`.
- LLM rendering mode is a closed two-value contract:
  - `streaming`: LLM is still generating and UI renders incrementally.
  - `snapshot`: LLM output is complete and UI renders once.
  - `protocol::ImageLlmRenderMode` keeps this axis type-safe and prevents extra ad-hoc modes.
- State-to-contract traceability:
  - `status` + `status_source` + `motion_source` + derived content source are projected as stable machine-readable markers for agent/runtime tooling.
- Snapshot baseline:
  - `Snapshot` is a required baseline capability for `Image`: component must render deterministically from complete upstream config/result input.
  - `Image` is snapshot-first (`data-ui-stream-support="optional"` + `data-ui-stream-fallback="snapshot"` + `data-ui-llm-mode="snapshot"`), and upper layers decide streaming orchestration.
- Streaming responsibility policy:
  - `Image` is not a long-form body-reading surface, so streaming is `optional` instead of required.
  - when upper layers do not provide incremental updates, fallback is explicitly `snapshot`.
  - output status is always explicit via `data-ui-output-status`; current stable baseline is `verified` and status axis is closed (`draft|verified|submittable`).
  - data validation, reconnect, and retry belong to upper layers; this component only renders semantic state deterministically.
- Safety boundary:
  - render path keeps explicit typed attributes only; no script injection or free-form HTML path is used.

## Rust Hygiene Contract

- Non-test source hygiene:
  - `components/image/src/*.rs` keeps `unwrap/expect` absent.
  - `components/image/src/*.rs` keeps `let _ = ...` swallowing pattern absent.
- String-copy hotspot policy:
  - class composition in `logic.rs` uses `Cow<'static, str>` (`compose_base_class`) and only allocates when a custom class suffix exists.
- Verification command:
  - `./scripts/check-rust-hygiene.sh`
  - current environment note: script execution in this container reports `PCRE2 is not available in this build of ripgrep`, which is tooling/runtime-specific and outside `Image` component source scope.
