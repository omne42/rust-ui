# Image

`Image` is a display component composed from `ui-state-primitives` + `ui-headless` + `ui-motion`.

## Goal / Non-goal / Risk Boundary

- Goal: provide a stable, accessible image primitive with fallback/skeleton/blur/zoom behavior.
- Non-goal: no business data fetching, no global state management, no theme orchestration in component layer.
- Risk boundary: if state/interaction rules drift across layers, fix them in primitives/headless/motion first, not in `view.rs` patches.

## Architecture Layers

- `logic.rs`: re-export only; consumes `ui-state-primitives::image` contracts.
- `view.rs`: Leptos structure render + locale attrs + semantic marker mount.
- `motion.rs`: `ImageMotion` contract, sanitize logic, wasm zoom spring driver, SSR no-op.
- `styles.rs`: static token-first CSS + reduced-motion fallback.
- `mod.rs`: minimal stable exports (`Image`, `ImageStatus`, `ImageRadius`, `ImageShadow`, motion helpers).

## API (Table)

### Image Props

| Prop | Type | Default |
| --- | --- | --- |
| `src` | `Option<String>` | `None` |
| `alt` | `String` | required |
| `fallback_src` | `Option<String>` | `None` |
| `disable_skeleton` | `bool` | `false` |
| `is_blurred` | `bool` | `false` |
| `is_zoomed` | `bool` | `false` |
| `radius` | `ImageRadius` (`Sm` / `Md` / `Lg` / `Full`) | `Lg` |
| `shadow` | `ImageShadow` (`None` / `Sm` / `Md`) | `Sm` |
| `motion` | `ImageMotion` | `ImageMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `node_ref` | `NodeRef<html::Img>` | empty ref |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | component does not expose explicit callbacks | `-` |

## Hello World

```rust
<Image
  src=Some("https://example.com/photo.jpg".to_string())
  alt="Cover".to_string()
/>
```

## Docs Playground 展示区

`apps/docs-app` 中的 `Image` 页面包含和 `button` 同类的工作台能力，分为三个区域：

- 展示（Display）：
  - `Image`（最小示例）
  - `Comparison Matrix: Loaded / Blurred / Fallback / Missing`（多场景对比）
- Config（配置面板）：
  - source（valid/invalid/missing）
  - radius、shadow、motion
  - zoomed、blurred、disable_skeleton、fallback、custom_class
- Code（代码面板）：
  - 根据当前配置实时生成可复制代码片段
- CSS Test（样式测试面板）：
  - 默认加载 `components/image/src/styles.rs` 的 CSS 合约
  - 支持在 playground 内 scoped css 调整验证
  - 同步输出 `ImageActualConfig` 作为配置快照

多场景对比用于快速验证关键路径：

- 有效图源 + 缩放
- 有效图源 + 模糊
- 无效图源 -> fallback
- 缺失图源 -> fallback

## Semantics and Accessibility

- `alt` is required by API.
- Locale plumbing is mounted via `lang` / `dir` on root wrapper.
- Stable semantic markers are exposed on root:
  - `data-slot="image-wrapper"`
  - `data-state` (`idle` / `loading` / `loaded` / `error`)
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

## State Primitive Contract

State contracts are centralized in `crates/ui-state-primitives/src/image.rs`:

- `ImageStatus`
- `ImageRadius`
- `ImageShadow`
- `ImageViewState`
- `resolve_view_state(...)`

Component layer only maps these outputs into render semantics.

## Agent Contract / Snapshot

- `Image` is a display primitive that supports snapshot rendering by default.
- Streaming protocol handling is out of scope for this component; upper layer owns stream state and recovery.
- Agent/test selectors should rely on stable `data-*` markers, not DOM depth/class guessing.
