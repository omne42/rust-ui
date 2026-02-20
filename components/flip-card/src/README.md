# FlipCard

`FlipCard` 是前后双面 3D 卡片组件，支持点击/键盘翻转、hover 翻转与可配置 spring 动效。

## Hello World（最小可用）

```rust
use ui_components::FlipCard;

view! {
    <FlipCard
        front=move || view! { <div>"Front"</div> }
        back=move || view! { <div>"Back"</div> }
    />
}
```

## docs-app 展示区（类似 button）

入口：`apps/docs-app/src/pages/components/pages/display_extra.rs::flip_card()`

- `Click + Keyboard Flip`
- `Interactive Playground (展示 / Config / Code / CSS Test)`
- `State + Source Markers`
- `Comparison Matrix (Default / Hover / Disabled / Dramatic Motion)`
- `Disabled`

## 展示区

- 实时渲染当前 `FlipCard` 配置（front/back 内容 + 交互状态）。
- 默认支持鼠标点击与键盘 `Enter` / `Space` 翻转。

## Config 区

在 Interactive Playground 里提供可调配置：

- `motion preset`（default / gentle / dramatic）
- `default_flipped`
- `flip_on_hover`
- `disabled`
- `custom id`
- `custom class`

## Code 区

- Playground 会生成 Copy-Paste Ready 代码片段（含必要 imports）。
- 配置变更会同步反映在代码片段中，便于复现当前状态。

## CSS Test 区

- 使用 scoped css live-edit（不污染页面其他 playground）。
- 基于 `components/flip-card/src/styles.rs` 原始样式进行覆盖测试。
- 同面板展示 `Actual config`，便于验证 class/data-state 选择器效果。

## 多场景对比显示

`Comparison Matrix` 同屏对比四种场景：

- Default
- Hover flip
- Disabled
- Dramatic motion（更强 hover scale + tilt）

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `front` | `ViewFn` | required |
| `back` | `ViewFn` | required |
| `default_flipped` | `bool` | `false` |
| `disabled` | `bool` | `false` |
| `flip_on_hover` | `bool` | `false` |
| `motion` | `FlipCardMotion` | `FlipCardMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `id` | `Option<String>` | `None`（自动生成） |

事件说明：

- 当前不暴露 `on_*` 回调；状态通过稳定语义标记对外可观测。

## 语义与样式契约

- 根节点：`role="button"`、`aria-pressed`、`aria-disabled`
- 稳定标记：`data-slot`、`data-state`、`data-visible`、`data-flip-mode`、`data-motion-source`、`data-class-source`、`data-id-source`
- 面片可见性：front/back 输出 `data-visible` / `data-hidden`

## 代码结构

- `mod.rs`：导出边界
- `logic.rs`：状态归一 + source 标记
- `view.rs`：结构与交互挂载
- `motion.rs`：动效契约 + wasm/non-wasm 分支
- `styles.rs`：静态 CSS 契约

## Source-first

- `components/flip-card/src/mod.rs`
- `components/flip-card/src/logic.rs`
- `components/flip-card/src/view.rs`
- `components/flip-card/src/motion.rs`
- `components/flip-card/src/styles.rs`
