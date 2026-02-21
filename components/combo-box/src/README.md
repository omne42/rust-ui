# ComboBox

`ComboBox` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion` 组合出来的输入筛选 + 列表选择组件。

## Quick Start (Hello World)

先跑默认路径，不需要先理解分层细节。

```rust
use leptos::prelude::*;
use ui_components::ComboBox;

let (selected, set_selected) = signal(None::<usize>);

view! {
    <ComboBox
        id_base="city".to_string()
        label="City".to_string()
        items=vec!["Tokyo".to_string(), "Osaka".to_string()]
        selected_index=selected
        set_selected_index=set_selected
    />
}
```

## 常见用法

- 受控 open：`is_open + on_open_change`
- 非受控 open：`default_open`
- 禁用：`is_disabled` / `disabled_indices`
- 校验：`is_required` / `is_invalid` + `description` / `error`

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控 open、可观测、可测试的组合输入交互基元。
- 非目标：不在组件层实现业务 store 绑定，不在组件层重写通用状态原语或键盘语义契约。
- 风险边界：状态机、A11y、动效若出现漂移，优先回迁到 `ui-state-primitives` / `ui-headless` / `ui-motion` 层修复。

## Architecture Layers

- `logic.rs`：props 归一化与状态派生（accessibility/open/root state、`RootDataState`、class 组合）。
- `view.rs`：Leptos 结构渲染与 headless 契约挂载（`use_combo_box`、`use_text_field`、`use_presence`、`use_popover_position`）。
- `motion.rs`：`ComboBoxMotion` 契约与动效参数清洗。
- `styles.rs`：静态 CSS 契约，样式依赖稳定 `data-*` / class 标记。
- `mod.rs`：最小导出面（`ComboBox`、`ComboBoxMotion`）。
- `ui-state-primitives/src/combo_box.rs`：纯状态原语（文本归一、索引映射、状态派生）。

## API (Table)

### ComboBox Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `label` | `String` | required |
| `items` | `Vec<String>` | required |
| `selected_index` | `ReadSignal<Option<usize>>` | required |
| `set_selected_index` | `WriteSignal<Option<usize>>` | required |
| `is_disabled` | `Option<bool>` | `None` |
| `disabled_indices` | `Vec<usize>` | `[]` |
| `is_required` | `Option<Signal<bool>>` | `None` |
| `is_invalid` | `Option<Signal<bool>>` | `None` |
| `aria_describedby` | `Signal<Option<String>>` | `None` |
| `description` | `Option<String>` | `None` |
| `error` | `Option<String>` | `None` |
| `placeholder` | `Option<String>` | `None` |
| `empty_message` | `Option<String>` | `None` |
| `toggle_button_aria_label` | `Option<String>` | `None` |
| `is_open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `motion` | `ComboBoxMotion` | `ComboBoxMotion::default()` |
| `class_name` | `Option<String>` | `None` |

## Controlled / Uncontrolled 契约

- open 轴遵循 triplet：`is_open` + `on_open_change` + `default_open`。
- 组件通过 `ui_headless::use_controllable_open_state_traced("combo-box", ...)` 统一受控/非受控行为。

## Migration

- `disabled` -> `is_disabled`
- `required` -> `is_required`
- `invalid` -> `is_invalid`
- `open` -> `is_open`

## Streaming 策略

- `Snapshot`：默认路径，组件稳定消费完整配置并渲染。
- `Streaming Optional`：`ComboBox` 不是 LLM 正文阅读面；若上层为流式容器，本组件按 `fallback=snapshot` 消费稳定配置。

## 展示 (Display)

- docs-app 页面：`apps/docs-app/src/pages/components/pages/collections.rs` 的 `combo_box()`。
- 展示区包含多场景对比：
  - 校验 + 禁用选项（invalid 与 disabled option 并存）
  - 受控 open（`is_open` + `on_open_change`）
  - 根禁用（`is_disabled=true`）
  - 空数据（`items=Vec::<String>::new()`）

## Config (Workbench Settings)

- Workbench 使用统一 `Playground controls` 面板调节：
  - `Invalid`
  - `Disabled root`
  - `Disable last option`
  - `Controlled open`
  - `Custom class marker`
- 目标是让状态切换可见、可重复、可回归，不把调参逻辑散落到多个示例。

## Code (Workbench Snippet)

- Workbench 通过 `code_signal` 输出与当前配置同步的可复制代码片段。
- 代码片段会按状态差异最小化输出，仅在配置开启时追加相关 props（例如 `is_open`、`disabled_indices`、`class_name`）。

## CSS Test (Scoped CSS)

- Workbench 通过 `test_css_source` 直接加载组件样式源：
  - `components/combo-box/src/styles.rs`
- `Playground` 的 CSS Test 面板支持局部样式覆写与还原，用于验证 CSS 契约是否稳定且可调。
- `test_config_signal` 同步输出实际配置，确保样式修改与运行状态可同时审查。

## Semantics and Accessibility

- input 使用 `role="combobox"`，list panel 使用 `role="listbox"` / `role="option"`。
- `aria-controls` / `aria-expanded` / `aria-activedescendant` 与 open/active 状态联动。
- 键盘处理由 `ui-headless::use_combo_box` 输出的 typed handlers 决定，不在视图层重写规则。
- 根节点输出稳定语义标记：`data-state`、`data-open`、`data-controlled`、`data-*-source` 等。

## Motion and Fallback

- panel 使用 popover motion，active option 高亮使用 active-highlight motion。
- `motion.rs::sanitize_motion` 会在挂载前清洗非法参数。
- non-wasm 路径依赖 `ui-motion` no-op/stub，保证 SSR/tooling 可编译。

## Testing Contract

- 语义契约测试：`components/combo-box/test/combo_box_semantics.rs`
- docs 页面示例：`apps/docs-app/src/pages/components/pages/collections.rs`
- 组件页 E2E 覆盖：`e2e/tests/docs_app_components_coverage.spec.mjs`

## Source-first

- 组件源码：`components/combo-box/src/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/combo_box.rs`
