# ColorSwatchPicker

`ColorSwatchPicker` 是颜色样本单选组件，默认走 snapshot 渲染路径，键盘与语义交互由 `ui-headless::use_radio` 提供。

阅读顺序建议：先看 `Hello World` 直接跑起来，再按需启用受控与高级配置。
默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。

## Hello World

```rust
use leptos::prelude::*;
use ui::{ColorSwatchPicker, ColorSwatchPickerItem};

view! {
    <ColorSwatchPicker
        swatches=signal(vec![ColorSwatchPickerItem::named("#f80", "Orange")]).0
    />
}
```

## 受控用法

```rust
use leptos::prelude::*;
use ui::{ColorSwatchPicker, ColorSwatchPickerItem};

let swatches = vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
];
let (selected, set_selected) = signal(Some("#A00".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected.set(next));

view! {
    <ColorSwatchPicker
        swatches=signal(swatches).0
        selected_color=selected
        on_selected_change=on_selected_change
        aria_label="Controlled swatch picker".to_string()
    />
}
```

## 常见用法（进阶）

- 受控/非受控轴：`selected_color + on_selected_change + default_selected_color`。
- 禁用分支：组件级 `is_disabled` 或 item 级 `ColorSwatchPickerItem::disabled(true)`。
- 视觉参数：`shape`、`rounding`、`is_bordered`、`class_name`。
- i18n/l10n：`lang` / `dir` 透传到 headless 语义契约。

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch_picker()`
- 组件源码：`components/color-swatch-picker/src/{mod,logic,view,styles,motion}.rs`
- package 模式前提：`component-color_swatch_picker`（运行时注入样式可叠加 `inject-css`）
