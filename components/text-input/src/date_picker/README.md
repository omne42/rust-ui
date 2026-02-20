# DatePicker

`DatePicker` 是一个由 `ui-logic-calendar` + `ui-headless` + `ui-motion` 组合的日期选择组件（按钮触发 + Popover 日历面板）。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的日期选择交互基元。
- 非目标：不在组件层实现业务 store、主题系统或跨组件动效引擎。
- 风险边界：跨层职责漂移时，优先回迁到对应层（state/headless/motion/theme），不在 `view.rs` 打补丁。

## Architecture Layers

- `logic.rs`：消费并 re-export `ui_logic_calendar::date_picker`，仅保留组件装配逻辑（`compose_class_name`）。
- `view.rs`：Leptos 结构渲染与 headless 契约挂载（controllable state、presence、aria 绑定）。
- `motion.rs`：`DatePickerMotion` 契约与 `sanitize_motion`，委托 `popover` 动效清洗。
- `styles.rs`：token-first 静态 CSS，只消费稳定语义标记与 `var(--ui-*)`。
- `mod.rs`：最小稳定导出（`DatePicker`、`DatePickerTone`、`DatePickerIds`、`DatePickerMotion`）。

## API (Table)

### DatePicker Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `year` | `i32` | required |
| `month` | `u8` | required（内部归一到 `1..=12`） |
| `tone` | `DatePickerTone` (`Default` / `Quiet` / `Strong`) | `Default` |
| `disabled` | `bool` | `false` |
| `open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `selected_day` | `Option<Signal<Option<u8>>>` | `None` |
| `default_selected_day` | `Option<u8>` | `None` |
| `on_selected_day_change` | `Option<Callback<Option<u8>>>` | `None` |
| `first_weekday` | `CalendarFirstWeekday` (`Sunday` / `Monday`) | `Sunday` |
| `show_outside_days` | `bool` | `false` |
| `popover_placement` | `PopoverPlacement` | `PopoverPlacement::BottomStart` |
| `motion` | `DatePickerMotion` | `DatePickerMotion::default()` |
| `placeholder` | `Option<String>` | `"Select date"` |
| `aria_label` | `Option<String>` | `"Date picker"` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `class_name` | `Option<String>` | `None` |

### DatePicker Events

| Event | Type | Default |
| --- | --- | --- |
| `on_open_change` | `Callback<bool>` | `None` |
| `on_selected_day_change` | `Callback<Option<u8>>` | `None` |

## Controlled / Uncontrolled Axes

- `open` 轴：`open + on_open_change + default_open`。
- `selected_day` 轴：`selected_day + on_selected_day_change + default_selected_day`。
- 两个状态轴都由 `ui_headless::use_controllable_state` / `use_controllable_open_state_traced` 统一管理。

## Hello World（最小可用）

```rust
<DatePicker
  id_base="release-date".to_string()
  year=2026
  month=3
/>
```

- 默认路径不需要用户手动接线 primitives/headless。
- 进阶能力（受控状态、自定义文案、自定义 motion）按需开启。

## Semantics and Accessibility

- 根节点使用 `role="group"`，并输出稳定语义标记：
  - `data-tone` / `data-state`
  - `data-open` / `data-closed` / `data-disabled`
  - `data-has-value` / `data-selected-day`
  - `data-placeholder-source` / `data-aria-source`
  - `data-class-source` / `data-motion-source`
- 触发器通过 `aria-haspopup="dialog"`、`aria-expanded`、`aria-controls` 与面板绑定。
- 面板使用 `role="dialog"` + `aria-labelledby`。
- 组件支持 `lang` / `dir` 接入（`ui_headless::a11y::locale_attrs`）。

## Motion and Fallback

- `DatePickerMotion` 当前承载 `popover: PopoverMotion`。
- 组件先调用 `sanitize_motion`，再把结果传给 `Popover`。
- non-wasm / SSR 路径依赖 `ui-motion` no-op/stub，保证编译与行为可预测。

## Tree-Shaking / Feature Gate

- 组件 feature：`component-date_picker`。
- 最小依赖特性链：`component-button` + `component-calendar` + `component-popover`。
- 样式通过 `css.rs` 在 feature 条件下聚合，不会无条件拉起全量组件 CSS。

## Testing Contract

- 组件语义回归：`crates/ui-components/tests/date_picker_semantics.rs`。
- primitives 回归：`crates/ui-logic-calendar/src/date_picker.rs` 单元测试。
- 最小特性验证建议：
  - `cargo test -p ui-components --test date_picker_semantics --no-default-features --features component-date_picker,inject-css`
  - `cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-date_picker,inject-css`

## Docs Playground（展示 / Config / Code / CSS Test）

- 展示区：实时预览 `DatePicker` 当前状态（受控 open、selected_day、tone、weekday、outside days、禁用态）。
- Config 区：通过按钮切换 month/open/disabled/tone/weekday/outside/custom motion/custom text。
- Code 区：`Playground` 动态输出当前配置的 copy-ready 代码片段。
- CSS Test 区：绑定 `components/text-input/src/date_picker/styles.rs`，显示当前实际配置字符串，便于样式契约回归。

### Comparison Matrix（多场景对比）

- `Comparison Matrix (Default / Quiet / Strong / Disabled)` 在 docs-app 同屏展示四种状态：
  - Default（含默认已选日期）
  - Quiet
  - Strong（含周起始切换）
  - Disabled（不可交互 + placeholder）
