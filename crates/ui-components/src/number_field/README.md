# NumberField

`NumberField` 是数字输入组件，组合 `ui-headless` 的 number-field/text-field/focus-ring 语义，并内置增减步进按钮。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控、可约束范围（`min/max/step`）的数字录入能力。
- 非目标：不负责业务校验规则与表单提交流程编排。
- 风险边界：数值裁剪、步进、解析逻辑集中在 `logic.rs`，避免散落在事件处理里。

## Architecture Layers

- `logic.rs`：`parse_i64`、`clamp_i64`、`step_i64` 等纯函数。
- `i18n.rs`：步进按钮可本地化文案（`Increment/Decrement`）。
- `view.rs`：挂载 headless a11y hooks，渲染输入框与步进按钮。
- `styles.rs`：基础样式与 `invalid/disabled/focus-visible` 状态样式。
- `mod.rs`：导出 `NumberField` 与逻辑 helper。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::NumberField;

let (value, set_value) = signal(1_i64);

view! {
    <NumberField
        id="qty".to_string()
        label="Quantity".to_string()
        value=value
        set_value=set_value
        min=Some(0)
        max=Some(99)
        step=1
    />
}
```

## API 约定

- 必填：`id`、`label`、`value`、`set_value`
- 常用可选：`disabled`、`min`、`max`、`step`、`on_change`
- 表单语义可选：`required`、`invalid`、`aria_describedby`、`description`、`error`
- 展示可选：`placeholder`、`class_name`、`node_ref`
- 语义观测：`data-focused`、`data-focus-visible`、`data-invalid`、`data-disabled`、`data-required`

## A11y / 交互

- 输入角色使用 spinbutton 语义（由 headless number-field 提供）。
- 键盘增减与按钮增减统一走同一套数值更新逻辑。
- `on_change` 在内部值更新后触发，便于外部联动。

## Source-first

- 组件源码：`crates/ui-components/src/number_field/{mod,logic,i18n,view,styles}.rs`
- 依赖组件：`crates/ui-components/src/button/`
- package feature：`component-number_field`（依赖 `component-button`，可选叠加 `inject-css`）

## Docs Playground

- docs-app `number-field` 页面已提供 `Interactive Playground`，包含展示区 + Config + Code + CSS Test。
- 同页包含多情况对比矩阵：默认/必填/错误/禁用状态。
