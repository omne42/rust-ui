# Description

`Description` is a text helper primitive with typed tone/element contracts and stable source markers.

## Goals / Non-goals / Risk Boundary

- Goal: provide predictable helper-text rendering with tone and truncation state.
- Non-goal: no field orchestration or form validation ownership in this component.
- Risk boundary: state priority (`disabled` vs `truncate`) must stay centralized in `logic.rs`.

## Architecture Layers

- `logic.rs`: normalizes text/aria values and resolves `DescriptionState`.
- `view.rs`: renders element variant (`span`/`p`/`div`) and mounts semantic markers.
- `styles.rs`: token-first static styles keyed by tone/state markers.
- `mod.rs`: exports minimal public API and state contracts.

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `text` | `String` | required (`DEFAULT_TEXT` fallback after normalization) |
| `tone` | `DescriptionTone` (`Default` / `Muted` / `Negative`) | `Default` |
| `disabled` | `bool` | `false` |
| `truncate` | `bool` | `false` |
| `element` | `DescriptionElement` (`Span` / `Paragraph` / `Div`) | `Paragraph` |
| `aria_label` | `Option<String>` | `DEFAULT_ARIA_LABEL` |
| `class_name` | `Option<String>` | `None` |

Events: none.

## Hello World

```rust
<Description text="This appears below the field.".to_string() />
```

## Docs Playground（展示区）

### 展示 (Display)

- workbench 预览 `Description` 在不同 tone/element/state 下的渲染。
- 直接观察 slot、state、tone 数据标记变化。

### config

- `Tone`：`default / muted / negative`。
- `Element`：`paragraph / span / div`。
- `Disabled`、`Truncate`、`Custom aria label`、`Custom class`。

### code

- `code` 面板按当前配置动态生成代码片段。
- 输出包含必要的枚举与状态参数，便于复制回归。

### css test

- `css test` 面板绑定 `components/description/src/styles.rs`。
- 支持在隔离范围内快速验证 tone 与 truncate 规则。

### 多场景对比显示

- `Tone Comparison`：对比 default/muted/negative。
- `Truncate + Disabled Comparison`：对比截断与禁用状态。

## Semantics and Accessibility

- Exposes stable semantic markers: `data-tone`, `data-state`, `data-disabled`, `data-truncate`, `data-aria-source`, `data-class-source`.
- `aria_label` supports custom label override; default label is normalized in logic.
- Slot contract is stable via `data-slot="description"` and `slot="description"`.

## Motion and Fallback

- No component-level motion contract.
- Behavior is static and deterministic for SSR/wasm paths.

## Source-first / Copy-Paste Ready

- Docs entry: `apps/docs-app/src/pages/components/pages/forms_extra.rs::description()`
- Source: `components/description/src/{mod,logic,view,styles}.rs`
- Package mode feature: `component-description` (optional `inject-css`)
