# ErrorView

`ErrorView` 用于展示字段级或局部表单错误信息，支持默认文本/自定义内容、可选图标与操作区、以及可配置的可见性动效合同。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::ErrorView;

view! {
    <ErrorView is_invalid=true message="Please enter a valid email".to_string() />
}
```

## 常用参数

- `is_invalid`: 控制错误区是否可见。
- `tone`: 视觉语义（默认 `Negative`）。
- `is_compact` / `compact`: 紧凑布局（推荐 `is_compact`，`compact` 为兼容别名）。
- `is_bordered` / `bordered`: 边框样式（推荐 `is_bordered`，`bordered` 为兼容别名）。
- `message`: 无 children 时的错误文本。
- `icon` / `actions`: 前置图标与操作区插槽。
- `lang` / `dir`: locale 语义透传。

## 文档入口

- docs-app: `/#/components/error-view`
- 源码: `crates/ui-components/src/error_view/`
