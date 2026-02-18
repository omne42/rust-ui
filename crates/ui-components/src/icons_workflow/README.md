# IconsWorkflow

`IconsWorkflow` is a workflow-namespace icon wrapper over `Iconset`, with normalized icon reference paths and stable source-state markers.

## Goals / Non-goals / Boundaries

- Goal: provide a typed workflow icon primitive with predictable fallback/default paths and observable contract markers.
- Non-goal: no business registry loading, no app-level icon orchestration, no custom rendering engine.
- Boundary: icon/class/aria/source normalization lives in `logic.rs`; `view.rs` only mounts normalized output.

## Architecture Layers

- `logic.rs`: icon reference normalization, optional text normalization, default workflow glyph registry, source/state derivation.
- `view.rs`: renders wrapper + inner `Iconset` and mounts stable `data-*` markers.
- `styles.rs`: static CSS selectors keyed by stable markers.
- `mod.rs`: minimal public API (`IconsWorkflow`, `IconsWorkflowSize`, `IconsWorkflowTone`, `IconsetGlyph`).

## API

| Prop | Type | Default |
| --- | --- | --- |
| `icon` | `String` | required (empty -> `"workflow:help"`) |
| `size` | `IconsWorkflowSize` | `Md` |
| `tone` | `IconsWorkflowTone` | `Default` |
| `disabled` | `bool` | `false` |
| `decorative` | `bool` | `true` |
| `aria_label` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `glyphs` | `Vec<IconsetGlyph>` | `[]` (extends built-in workflow glyphs) |

## State / Source Contract

`IconsWorkflow` exposes stable markers:

- `data-slot="icons-workflow"`
- `data-state` (`ready` / `disabled` / `decorative`)
- `data-icon-reference`, `data-icon-reference-source` (`default` / `prefixed` / `explicit`)
- `data-aria-source`, `data-class-source`, `data-glyph-source`, `data-size-source`, `data-tone-source`
- `data-disabled`, `data-decorative`, `data-custom-class`, `data-custom-glyphs`, `data-custom-size`, `data-custom-tone`

## Hello World

```rust
<IconsWorkflow
  icon="success".to_string()
  size=IconsWorkflowSize::Md
  tone=IconsWorkflowTone::Accent
  decorative=false
/>
```

## docs-app Playground 区块

- 展示区: current 与 baseline 并排展示，验证 icon/source/state 行为差异。
- Config 区: 交互切换 icon reference、size/tone、disabled/decorative、aria/class/glyph source。
- Code 区: 根据当前配置动态生成可复制 `IconsWorkflow` 代码。
- CSS Test 区: 加载 `crates/ui-components/src/icons_workflow/styles.rs` 做 scoped CSS 合同验证。

## 对比场景

| 场景 | 对比点 |
| --- | --- |
| Built-in Workflow Glyphs | 默认 workflow glyph 与 tone 对比 |
| Custom Workflow Extension | 自定义 glyph registry + class source |
| State + Source Markers | `data-state` + source markers 合同检查 |
| Interactive Playground | current config vs baseline |

## Accessibility

- 非 decorative 路径支持显式 `aria_label`。
- decorative/disabled/source 状态可通过稳定 marker 直接检索。
- 支持默认与自定义 glyph label 合同回归测试。
