# Slider

`Slider` 是范围输入组件，默认走 snapshot 渲染路径，交互语义由 `ui-headless::use_slider` 提供。

## Hello World

```rust
use ui_components::Slider;

view! { <Slider label="Volume".to_string() default_value=36.0 /> }
```

## 受控用法

```rust
use leptos::prelude::*;
use ui_components::Slider;

let (value_raw, set_value_raw) = signal(36.0_f64);
let value = Signal::derive(move || value_raw.get());
let on_value_change = Callback::new(move |next: f64| set_value_raw.set(next));

view! { <Slider value=value on_value_change=on_value_change default_value=20.0 /> }
```

## API 约定

- 受控/非受控轴：`value` + `on_value_change` + `default_value`
- 布尔状态：`is_disabled`
- 兼容迁移：保留 `disabled` / `set_value` / `on_change`，内部统一映射到新命名
- i18n/l10n：`lang` / `dir` 透传到 headless 语义契约
- 语义观测：根节点输出稳定 `data-*` / `aria-*` / `data-ui-*` 标记

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_extra.rs::slider()`
- 组件源码：`components/slider/src/{mod,logic,view,styles,motion}.rs`
- package 模式前提：`component-slider`（样式注入可选叠加 `inject-css`）
