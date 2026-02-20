# Overlays

`Overlays` 是 overlay 家族的聚合入口，统一暴露 `Overlay`、`Popover`、`Modal`、`Tray`，并提供 `OverlaysRoot` 作为稳定语义容器。

## 目标 / 非目标 / 风险边界

- 目标：为 overlay 家族提供一致的导出边界、根容器语义标记和可测试状态来源标记。
- 非目标：不在本层重写 `Overlay/Popover/Modal/Tray` 的业务交互语义与状态机。
- 风险边界：若某个子组件语义漂移，优先在对应组件层修复，不在 `OverlaysRoot` 追加补丁逻辑掩盖问题。

## Architecture Layers

- `mod.rs`：聚合导出边界，重导出 `Overlay`/`Popover`/`Modal`/`Tray` 及 motion 类型。
- `logic.rs`：`OverlaysRoot` 的输入归一、状态派生、来源标记（`data-state`、`data-layer`、`data-*-source`）。
- `view.rs`：`OverlaysRoot` 的 Leptos 结构渲染与语义挂载。
- `motion.rs`：`OverlaysMotion` 聚合契约，委托到 overlay/popover/tray 的 sanitize 逻辑。
- `styles.rs`：仅根容器静态 CSS 状态选择器，不承载子组件视觉规则。

## API (Table)

### OverlaysRoot Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `id_base` | `Option<String>` | `None`（归一到 `"overlays-root"`） |
| `open` | `bool` | `false` |
| `modal` | `bool` | `false` |
| `class_name` | `Option<String>` | `None` |

### Re-exports

- `Overlay`, `OverlayMotion`
- `Popover`, `PopoverMotion`
- `Modal`
- `Tray`, `TrayMotion`
- `OverlaysMotion`

## Hello World（最小可用）

```rust
use leptos::prelude::*;
use ui_components::{Overlay, OverlaysRoot};

let (open, set_open) = signal(true);
let open_signal = Signal::derive(move || open.get());
let on_close = Callback::new(move |_| set_open.set(false));

view! {
  <OverlaysRoot open=open_signal.get() modal=true>
    <Overlay open=open_signal on_close=on_close>
      <div>"Overlay content"</div>
    </Overlay>
  </OverlaysRoot>
}
```

## Semantics and Accessibility

- `OverlaysRoot` 根节点固定输出 `role="group"`、`aria-label="Overlays"`。
- 暴露稳定语义标记：`data-slot="overlays"`、`data-state`、`data-layer`、`data-id-source`、`data-class-source`。
- 额外来源位通过布尔标记输出：`data-custom-id`、`data-custom-class`。

## Motion and Fallback

- `OverlaysMotion` 只做聚合，不实现底层动效引擎。
- sanitize 委托给 `overlay::motion`、`popover::motion`、`tray::motion`，保持参数归一一致性。
- 非 wasm 与 reduced-motion 的降级策略由下层组件各自负责。

## Source-first / Copy-Paste Ready

- docs 入口：
  - `apps/docs-app/src/pages/components/pages/overlays.rs`
  - `apps/docs-app/src/pages/components/pages/overlays_extra.rs`
- 组件源码：`components/overlays/src/{mod,logic,view,styles,motion}.rs`
- package 模式 feature：`component-overlays`（样式注入可选叠加 `inject-css`）
