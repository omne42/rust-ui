# Tray

`Tray` 是基于 `Sheet` 组合的底部抽屉组件，使用 `ui-state-primitives::tray` 派生状态语义，并输出稳定 `data-*` 来源标记。

## 目标 / 非目标 / 风险边界

- 目标：提供可关闭、可配置 footer/description、可观测来源标记的底部抽屉组件。
- 非目标：不在组件层重写通用 overlay/presence 状态机，不承载全局业务状态管理。
- 风险边界：若出现跨组件重复语义，优先下沉到 `ui-state-primitives` 或 `ui-headless`，避免在 `view.rs` 累积条件分支。

## Architecture Layers

- `logic.rs`：复用 `ui_state_primitives::tray`，负责输入归一与类名组合。
- `view.rs`：挂载 `Sheet`、A11y 属性与稳定 `data-*` 语义字段。
- `motion.rs`：`TrayMotion` 适配 `SheetMotion`，并委托 sanitize。
- `styles.rs`：静态 CSS 与状态选择器（description/footer/size/dismiss/source markers）。
- `mod.rs`：最小导出面（`Tray`、`TrayMotion`、状态类型）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `open` | `Signal<bool>` | required |
| `on_close` | `OnPress` | required |
| `id_base` | `String` | required（空白归一为 `"ui-tray"`） |
| `title` | `String` | required（空白归一为 `"Tray"`） |
| `children` | `ChildrenFn` | required |
| `description` | `Option<String>` | `None` |
| `footer` | `Option<ViewFn>` | `None` |
| `motion` | `TrayMotion` | `TrayMotion::default()` |
| `show_close_button` | `bool` | `true` |
| `close_label` | `&'static str` | `"Close tray"` |
| `is_fixed_height` | `bool` | `false` |
| `is_dismissable` | `bool` | `true` |
| `is_keyboard_dismiss_disabled` | `bool` | `false` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
use leptos::prelude::*;
use ui_components::Tray;

let (open, set_open) = signal(true);
let open_signal = Signal::derive(move || open.get());
let on_close = Callback::new(move |_| set_open.set(false));

view! {
  <Tray
    open=open_signal
    on_close=on_close
    id_base="docs-tray".to_string()
    title="Notifications".to_string()
  >
    <p>"Tray body"</p>
  </Tray>
}
```

## Semantics and Accessibility

- `Tray` 通过 `overlay_dialog_attrs` 生成 `aria-labelledby` / `aria-describedby` / `lang` / `dir`。
- 根节点暴露完整状态和来源字段，覆盖 description/footer/close/size/dismiss/motion/exit 等轴。
- 关闭按钮通过 `Button`（`is_icon_only=true`）暴露可访问标签（`close_label`）。

## Motion and Fallback

- `TrayMotion` 当前封装 `sheet: SheetMotion`。
- `sanitize_motion` 复用 `sheet::motion::sanitize_motion`，对无效数值做归一。
- 非 wasm 与 reduced-motion 的基础降级能力由 `Sheet` 与底层 motion 层负责。

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/overlays_extra.rs::tray()`
- 组件源码：`components/tray/src/{mod,logic,view,styles,motion}.rs`
- package 模式 feature：`component-tray`（样式注入可选叠加 `inject-css`）
