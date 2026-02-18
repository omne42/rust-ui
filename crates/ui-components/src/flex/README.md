# Flex

`Flex` is a token-first layout primitive with typed axis controls, stable state/source markers, and optional motion contract.

## Goals / Non-goals / Boundaries

- Goal: expose a consistent flex layout API with typed direction/wrap/align/justify/gap inputs.
- Non-goal: no app-level layout orchestration, no business state ownership, no style-in-view branching.
- Boundary: all layout normalization and source tracking stays in `logic.rs`; `view.rs` only mounts resolved state.

## Architecture Layers

- `logic.rs`: enum-typed layout axes, aria fallback, state/source derivation.
- `view.rs`: renders root `<div>`, mounts markers, and wires `motion::attach_motion`.
- `motion.rs`: `FlexMotion` contract + platform-safe no-op attach.
- `styles.rs`: static selectors keyed by `data-*` and stable classes.
- `mod.rs`: minimal exports (`Flex`, axis enums, `FlexMotion`, `DEFAULT_ARIA_LABEL`).

## API

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `direction` | `FlexDirection` | `Row` |
| `wrap` | `FlexWrap` | `NoWrap` |
| `justify` | `FlexJustify` | `Start` |
| `align` | `FlexAlign` | `Stretch` |
| `gap` | `FlexGap` | `Sm` |
| `inline` | `bool` | `false` |
| `motion` | `FlexMotion` | `FlexMotion::default()` |
| `aria_label` | `Option<String>` | fallback to `DEFAULT_ARIA_LABEL` (`"Flex"`) |
| `class_name` | `Option<String>` | `None` |

## Motion Contract

- `FlexMotion { animate_in: bool }`
- `sanitize_motion` is identity (typed contract stays explicit).
- `attach_motion` is a no-op on both wasm/non-wasm paths, preserving SSR/tooling compatibility.

## State / Source Contract

`Flex` exposes stable markers:

- `data-slot="flex"`
- `data-direction`, `data-wrap`, `data-justify`, `data-align`, `data-gap`
- `data-inline`
- `data-state` (derived layout bucket)
- `data-aria-source`, `data-custom-class`, `data-class-source`
- `data-motion-source`, `data-custom-motion`

## Hello World

```rust
<Flex>
  <div>"A"</div>
  <div>"B"</div>
</Flex>
```

## docs-app Playground 区块

- 展示区: 当前配置与 baseline 并排显示，便于观察布局轴变化。
- Config 区: 切换 direction/wrap/inline/distribution/custom class。
- Code 区: 输出与当前配置一致的 `Flex` 代码片段。
- CSS Test 区: 加载 `crates/ui-components/src/flex/styles.rs` 并支持 scoped CSS 实验。

## 对比场景

| 场景 | 对比点 |
| --- | --- |
| Direction + Wrap + Gap | 行列方向与换行差异 |
| Inline + Distribution | inline + 对齐/分布策略差异 |
| Interactive Playground | current config vs baseline |

## Accessibility

- Supports explicit/custom `aria-label` with normalized fallback.
- Marker contract keeps aria/class/motion source inspectable for tests and automation.
