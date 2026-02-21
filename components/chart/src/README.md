# Chart

`Chart` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-visual-primitive` 组合出来的图表组件，提供 `Bar/Line` 两种展示模式与可受控 active-index 语义。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的图表交互基元（hover/keyboard/action）。
- 非目标：不在组件层承载业务数据拉取、全局状态管理或主题系统实现。
- 风险边界：跨层职责漂移（state/headless/motion/theme）时，优先回迁到对应层，不在 `view.rs` 叠补丁。

## Architecture Layers

- `logic.rs`：仅 re-export `ui-state-primitives::chart` 能力（归一化、状态派生、几何映射、键盘索引规则）。
- `motion.rs`：`ChartMotion` 契约映射与 attach（复用 `active_highlight`，wasm/non-wasm 均可安全降级）。
- `view.rs`：Leptos 结构渲染与装配（挂载 `use_chart` 语义契约、受控状态、事件回调）。
- `styles.rs`：仅静态 CSS 契约，视觉值由 `var(--ui-*)` 驱动。
- `mod.rs`：公开最小稳定 API（`Chart`、`ChartKind`、`ChartPoint`、`ChartMotion`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `points` | `Vec<ChartPoint>` | required |
| `id_base` | `Option<String>` | `"ui-chart"` |
| `kind` | `ChartKind` (`Bar` / `Line`) | `Bar` |
| `active_index` | `Option<Signal<usize>>` | `None` |
| `default_active_index` | `Option<usize>` | `Some(0)` |
| `on_active_index_change` | `Option<Callback<usize>>` | `None` |
| `on_action` | `Option<Callback<String>>` | `None` |
| `is_disabled` | `bool` | `false` |
| `is_show_grid` | `bool` | `true` |
| `motion` | `ChartMotion` | `ChartMotion::default()` |
| `aria_label` | `Option<String>` | `"Chart"` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### Events

| Event | Type | Description |
| --- | --- | --- |
| `on_active_index_change` | `Callback<usize>` | active index 变化（受控模式回传） |
| `on_action` | `Callback<String>` | 激活动作回传 point `id` |

## Hello World（最小可用）

```rust
<Chart points=vec![ChartPoint::new("jan", "Jan", 12.0), ChartPoint::new("feb", "Feb", 18.5), ChartPoint::new("mar", "Mar", 17.2)] />
```

- 默认路径不需要手动接线 `ui-state-primitives` / `ui-headless`。
- 先传 `points` 即可运行，后续再按需开启受控、动作、动效等高级参数。

## 展示区（Display）

- docs-app 现提供 5 组展示：
  - `Hello World`
  - `Interactive Playground (展示 / Config / Code / CSS Test)`
  - `Comparison Matrix (Bar / Line / Disabled / Empty)`
  - `Bar + Hover/Keyboard + Action`
  - `Controlled Line + Active Index`
- 对比维度覆盖：`Bar vs Line`、`enabled vs disabled`、`has data vs empty`、`uncontrolled vs controlled`。

## Config 展示区

- `Interactive Playground` 的设置面板提供：
  - `kind` 切换（bar / line）
  - `dataset` 切换（revenue / growth / flat）
  - `is_disabled` / `is_show_grid` / `custom class` / `lang` 开关
- 同步输出 `Actual config` 文本，包含当前 class/marker 预期，方便契约核对。

## Code 展示区

- `Playground` 内置 `Show code` 面板，支持一键复制可运行片段。
- `Interactive Playground` 的 code 为动态生成，会随配置变化同步更新。

## CSS Test 展示区

- `Interactive Playground` 启用 `Scoped CSS` 测试面板：
  - Source: `components/chart/src/styles.rs`
  - 预加载 `ui_components::chart::styles::CSS`
  - 支持局部改写与 `Restore original CSS` 回滚
- 用于验证样式契约是否仍由稳定 `data-*` / class 驱动。

## 常见用法

```rust
let (last_action, set_last_action) = signal("none".to_string());

<Chart
  id_base="revenue-chart".to_string()
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
    ChartPoint::new("apr", "Apr", 24.7),
  ]
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
```

## 再进阶（受控 + 语义 + 动效）

```rust
let (active_raw, set_active_raw) = signal(1_usize);

<Chart
  id_base="growth-line".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  active_index=Signal::derive(move || active_raw.get())
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
  is_disabled=false
  aria_label="Quarterly growth line chart".to_string()
  class_name="docs-chart-custom".to_string()
  lang="en-US".to_string()
/>
```

## Semantics and Accessibility

- 根节点使用 `role="region"` + `aria-label`，并支持 `lang/dir`。
- 键盘语义通过 `ui_headless::use_chart` 统一输出（`Arrow*`/`Home`/`End`/`Enter`/`Space`）。
- 稳定语义标记：`data-kind`、`data-state`、`data-controlled`、`data-active-index`、`data-class-source`、`data-motion-source` 等。

## Motion and Fallback

- `ChartMotion` 复用 `active_highlight` 合同，legend 高亮由 `chart/motion.rs` 统一 attach。
- 非 wasm 路径为安全 no-op，保证 SSR/tooling 编译可预测。
- reduced-motion 降级由底层 motion 能力统一处理。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/display_extra.rs`
- `chart()` 页面包含：
  - `Bar + Hover/Keyboard + Action`
  - `Controlled Line + Active Index`

## Source-first Copy-Paste Ready

- docs-app Playground 自带复制按钮（`apps/docs-app/src/playground.rs`）。
- 常见导入：
  - `use leptos::prelude::*;`
  - `use ui_components::*;`
- 真实源码落点：
  - `components/chart/src/mod.rs`
  - `components/chart/src/logic.rs`
  - `components/chart/src/view.rs`
  - `components/chart/src/styles.rs`
  - `components/chart/src/motion.rs`
