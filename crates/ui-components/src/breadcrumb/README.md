# Breadcrumb

`Breadcrumb` is a primitive breadcrumb family composed from `ui-state-primitives` state contracts and `ui-components` view assembly.

## Goals / Non-goals / Risk Boundary

- Goal: provide a stable, accessible breadcrumb composition API with explicit slots.
- Non-goal: no app-level routing/state management inside the component.
- Risk boundary: if state/source markers drift, fix in `logic.rs` first, not with ad-hoc `view.rs` conditionals.

## Architecture Layers

- `logic.rs`: text normalization and state/source derivation via `ui_state_primitives::breadcrumb`.
- `view.rs`: renders `Breadcrumb*` slots and mounts stable `data-*` / `aria-*` markers.
- `styles.rs`: static token-first CSS.
- `mod.rs`: exports minimal public API (`Breadcrumb`, `BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`, `BreadcrumbSeparator`, `BreadcrumbEllipsis`).

## API (Table)

### `Breadcrumb` Props

| Prop | Type | Default |
| --- | --- | --- |
| `aria_label` | `Option<String>` | `DEFAULT_ARIA_LABEL` |
| `class_name` | `Option<String>` | `None` |
| `children` | `Children` | required |

### Slot Props

| Component | Props |
| --- | --- |
| `BreadcrumbList` | `class_name`, `children` |
| `BreadcrumbItem` | `class_name`, `children` |
| `BreadcrumbLink` | `href`, `class_name`, `children` |
| `BreadcrumbPage` | `class_name`, `children` |
| `BreadcrumbSeparator` | `class_name`, `children` (`None` => `"/"`) |
| `BreadcrumbEllipsis` | `class_name` |

## Hello World

```rust
<Breadcrumb>
  <BreadcrumbList>
    <BreadcrumbItem><BreadcrumbLink href="/">"Home"</BreadcrumbLink></BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbItem><BreadcrumbPage>"Current"</BreadcrumbPage></BreadcrumbItem>
  </BreadcrumbList>
</Breadcrumb>
```

## Docs Playground（展示区）

### 展示 (Display)

- 预览 `trail / label_only / empty` 三种 breadcrumb 场景。
- 同时展示链接项、当前页项、空列表行为。

### config

- `Scenario`：切换 trail、仅文本、空状态。
- `Custom aria label`：验证无障碍标签来源切换。

### code

- `code` 面板实时输出当前配置对应的可复制代码。
- 仅在非默认参数下输出额外 prop，避免样板噪音。

### css test

- `css test` 面板绑定 `crates/ui-components/src/breadcrumb/styles.rs`。
- 可在隔离作用域下调整样式并观察 `data-*` 契约是否稳定。

### 多场景对比显示

- 对比 `Label-Only` 与 `Empty` 两个状态，验证无链接场景和 0 项场景。

## Semantics and Accessibility

- Root renders `nav` with `aria-label`.
- Current page renders `aria-current="page"`.
- Separator and ellipsis render presentational semantics (`aria-hidden`, `role="presentation"`).
- Stable markers include `data-state`, `data-aria-source`, `data-class-source`, `data-href-state`, `data-content-source`.

## Motion and Fallback

- No dedicated motion contract in this primitive family.
- Styling remains static and predictable across SSR/wasm.

## Source-first / Copy-Paste Ready

- Docs entry: `apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs` and `apps/docs-app/src/pages/components/pages/collections_breadcrumb_primitives.rs`
- Source: `crates/ui-components/src/breadcrumb/{mod,logic,view,styles}.rs`
- Package mode feature: `component-breadcrumb` (optional `inject-css`)
