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
| `is_open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None`（归一化为 `false`） |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `on_close` | `Option<OnPress>` | `None` |
| `id_base` | `String` | required（空白归一为 `"ui-tray"`） |
| `title` | `String` | required（空白归一为 `"Tray"`） |
| `children` | `ChildrenFn` | required |
| `description` | `Option<String>` | `None` |
| `footer` | `Option<ViewFn>` | `None` |
| `motion` | `Option<TrayMotion>` | `None`（归一化为 `TrayMotion::default()`） |
| `is_show_close_button` | `Option<bool>` | `None`（归一化为 `true`） |
| `close_label` | `Option<&'static str>` | `None`（归一化为 `"Close tray"`） |
| `is_fixed_height` | `Option<bool>` | `None`（归一化为 `false`） |
| `is_dismissable` | `Option<bool>` | `None`（归一化为 `true`） |
| `is_keyboard_dismiss_disabled` | `Option<bool>` | `None`（归一化为 `false`） |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |
| `class_name` | `Option<String>` | `None` |

## Naming Migration

- `open` 已统一为 `is_open`（布尔状态轴统一 `is_*`）。
- `show_close_button` 已统一为 `is_show_close_button`（布尔状态轴统一 `is_*`）。
- 迁移方式：将所有 `<Tray open=... show_close_button=...>` 调整为 `<Tray is_open=... is_show_close_button=...>`。

## Open State Contract

- 默认值优先级统一在 `logic.rs::normalize_defaults` 与 `logic.rs::normalize_open_state` 归一化，`view.rs` 仅消费归一化输出。
- 受控：`is_open + on_open_change`，外部状态为单一事实来源。
- 非受控：仅传 `default_open` 初始化，后续由组件内部状态管理。
- `on_close` 是可选副作用回调，不替代 `on_open_change` 状态轴。

## Hello World（最小可用）

```rust
use leptos::prelude::*;
use ui::Tray;

view! {
  <Tray default_open=true id_base="docs-tray".to_string() title="Notifications".to_string()>
    <p>"Tray body"</p>
  </Tray>
}
```

## Semantics and Accessibility

- `Tray` 通过 `use_tray_a11y(TrayA11yOptions)` 生成 `aria-labelledby` / `aria-describedby` / `lang` / `dir`。
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
