# ColorPicker

`ColorPicker` 是颜色选择交互组件，默认走 snapshot 渲染路径，组合 `ColorSwatchPicker` 作为常见内容区。

阅读顺序建议：先看 `Hello World（默认路径）` 直接跑起来，再按需启用受控与高级配置。
默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。

## Hello World（最小可用）

```rust
use leptos::prelude::*;
use ui::ColorPicker;

view! {
    <ColorPicker id_base="demo-color-picker".to_string()>
        <div class="ui-muted">"Default picker content"</div>
    </ColorPicker>
}
```

## 受控用法

```rust
use leptos::prelude::*;
use ui::{ColorPicker, ColorSwatchPicker, ColorSwatchPickerItem};

let (selected_color, set_selected_color) = signal(Some("#ef4444".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));
let (open, set_open) = signal(false);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));
let swatches = vec![
    ColorSwatchPickerItem::named("#ef4444", "Red"),
    ColorSwatchPickerItem::named("#3b82f6", "Blue"),
];
let selected_color_signal: Signal<Option<String>> = selected_color.into();
let open_signal: Signal<bool> = open.into();

view! {
    <ColorPicker
        id_base="demo-color-picker-controlled".to_string()
        label="Fill".to_string()
        selected_color=selected_color_signal
        on_selected_change=on_selected_change
        open=open_signal
        on_open_change=on_open_change
    >
        <ColorSwatchPicker
            swatches=swatches
            selected_color=selected_color_signal
            on_selected_change=on_selected_change
        />
    </ColorPicker>
}
```

## 常见用法（进阶）

- 受控/非受控轴：`value + on_value_change + default_value`、`selected_color + on_selected_change + default_selected_color`、`open + on_open_change + default_open`。
- 状态控制：`is_disabled` 为标准命名，`disabled` 作为兼容别名由内部归一化。
- 国际化：`lang` / `dir` 透传到 headless 语义契约。
- 语义观测：根节点输出稳定 `data-*` / `aria-*` / `data-ui-*` 标记，便于测试与自动化消费。

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_picker()`
- 组件源码：`components/color-picker/src/{mod,logic,view,styles,motion}.rs`
- package 模式前提：启用 `component-color_picker`（运行时注入样式时可叠加 `inject-css`）。
