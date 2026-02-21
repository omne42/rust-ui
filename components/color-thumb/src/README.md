# ColorThumb

`ColorThumb` 是颜色拾取类组件里的锚点原语，负责输出稳定的焦点/拖拽/禁用语义标记，并保持默认快照渲染路径可直接使用。

阅读顺序建议：先跑 `Hello World`，再看常见状态组合，最后按需启用进阶参数。
默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。

## Hello World

```rust
use ui_components::ColorThumb;

let board_style =
    "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed var(--ui-border);";

view! {
    <div style=board_style>
        <ColorThumb id_base="demo-color-thumb".to_string() />
    </div>
}
```

## 常见用法

```rust
use ui_components::ColorThumb;

view! {
    <div style=board_style>
        <ColorThumb
            id_base="demo-color-thumb-focused".to_string()
            color="#10b981".to_string()
            is_focused=true
            x_percent=52.0
            y_percent=44.0
        />
        <ColorThumb
            id_base="demo-color-thumb-dragging".to_string()
            color="#3b82f6".to_string()
            is_dragging=true
            x_percent=82.0
            y_percent=28.0
        />
        <ColorThumb
            id_base="demo-color-thumb-disabled".to_string()
            color="#a78bfa".to_string()
            is_disabled=true
            is_loupe_visible=false
            x_percent=30.0
            y_percent=56.0
        />
    </div>
}
```

## 进阶参数

- 视觉与语义输入：`color`、`class_name`、`aria_label`、`aria_value_text`
- 状态输入：`is_disabled`、`is_focused`、`is_dragging`、`is_loupe_visible`
- 位置输入：`x_percent`、`y_percent`（越界值会在逻辑层归一化）
- 控制模型说明：`ColorThumb` 不提供 `value/default_value/on_value_change` 受控轴，属于外部 props 驱动组件

## Source-first / Copy-Paste Ready

- 复制入口：docs Playground 的 `Show code + Copy`（自动补 imports）
- 复制补全链路：`apps/docs-app/src/playground.rs::compose_copy_ready_code`
- docs 入口：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_thumb()`
- 组件源码：`components/color-thumb/src/{mod,logic,view,styles,motion}.rs`
- package 特性前提：`component-color_thumb`（运行时注入 CSS 可叠加 `inject-css`）
