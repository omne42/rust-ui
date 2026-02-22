# ColorWheel

`ColorWheel` 是一个用于选择色相角度（0°~360°）的输入组件。默认路径只要一个组件实例就能工作，不需要先理解分层架构。

## Hello World

```rust
use leptos::prelude::*;
use ui::ColorWheel;

view! {
    <ColorWheel id_base="demo-color-wheel".to_string() />
}
```

## 常见用法

1. 基础默认用法
   - 只传 `id_base`，组件使用默认 `step`、默认标签与默认值归一化。
2. 受控/非受控
   - 受控：`value + on_value_change`
   - 非受控：`default_value`
3. 禁用与可见性
   - `is_disabled=true` 控制禁用态
   - `is_value_label_visible=false` 控制值标签显示

## 新手路径（先用起来，再进阶）

1. 先跑默认路径：`<ColorWheel id_base=... />`
2. 再加常见参数：`default_value`、`is_disabled`、`label`、`aria_label`
3. 最后再用进阶参数：`motion`、`class_name`、`lang`、`dir`

## 文档入口

- docs-app 页面：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()`
- Playground 顺序为：`Hello World -> State Matrix -> Parameter Matrix -> Controlled vs Uncontrolled -> Interactive Workbench`

## Source-first

- 组件源码：`components/color-wheel/src/{mod,logic,view,styles,motion}.rs`
- package feature：`component-color_wheel`（可选叠加 `inject-css`）
- docs 复制链路：`apps/docs-app/src/playground.rs::compose_copy_ready_code`（复制时自动补 imports，降低“复制即报错”风险）
