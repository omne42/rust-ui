# ColorSlider

`ColorSlider` 是单通道颜色滑杆组件，默认走 snapshot 渲染路径，交互语义由 `ui-headless::use_slider` 提供。

## Hello World

```rust
use ui_components::{ColorSlider, ColorSliderChannel};

view! {
    <ColorSlider
        id_base="demo-color-slider".to_string()
        channel=ColorSliderChannel::Hue
        default_value=220.0
    />
}
```

## 受控用法

```rust
use leptos::prelude::*;
use ui_components::{ColorSlider, ColorSliderChannel};

let (hue_raw, set_hue_raw) = signal(220.0_f64);
let hue = Signal::derive(move || hue_raw.get());
let on_hue_change = Callback::new(move |next: f64| set_hue_raw.set(next));

view! {
    <ColorSlider
        id_base="demo-color-slider-hue".to_string()
        channel=ColorSliderChannel::Hue
        value=Some(hue)
        on_value_change=Some(on_hue_change)
        default_value=220.0
    />
}
```

## API 约定

- 受控/非受控轴：`value` + `on_value_change` + `default_value`
- 布尔状态：`is_disabled`
- 兼容迁移：保留 `disabled`，内部统一映射到 `is_disabled`
- i18n/l10n：`lang` / `dir` 透传到 headless 语义契约
- 语义观测：根节点输出稳定 `data-*` / `aria-*` / `data-ui-*` 标记

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_slider()`
- 组件源码：`crates/ui-components/src/color_slider/{mod,logic,view,styles,motion}.rs`
- package 模式前提：`component-color_slider`（样式注入可选叠加 `inject-css`）
