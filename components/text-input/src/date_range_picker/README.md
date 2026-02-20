# DateRangePicker

`DateRangePicker` 是一个基于 `ui-state-primitives` + `ui-headless` + `DatePicker` 组合出来的日期区间选择组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的日期区间选择能力。
- 非目标：不在组件层实现业务级日历存储、远程校验或全局时间策略。
- 风险边界：区间有效性与状态归一化必须在 primitives/logic 层完成，不在 `view.rs` 追加分支修补。

## Architecture Layers

- `crates/ui-logic-calendar/src/date_range_picker.rs`：`DateRangePickerStateInput`、`DateRangePickerState`、区间有效性与来源标记派生。
- `logic.rs`：消费 primitives，并统一文本类默认值（label/placeholder/aria/hint）。
- `view.rs`：Leptos 结构渲染，组合两个 `DatePicker`，挂载语义标记。
- `styles.rs`：仅静态 CSS 契约，使用 `var(--ui-*)`。
- `mod.rs`：公开最小稳定 API（`DateRangePicker`、`DateRangePickerTone`）。

## API (Table)

### DateRangePicker Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `start_year` | `i32` | required |
| `start_month` | `u8` | required（自动归一到 `1..=12`） |
| `end_year` | `i32` | required |
| `end_month` | `u8` | required（自动归一到 `1..=12`） |
| `tone` | `DateRangePickerTone` (`Default` / `Quiet` / `Strong`) | `Default` |
| `disabled` | `bool` | `false` |
| `start_day` | `Option<Signal<Option<u8>>>` | `None` |
| `default_start_day` | `Option<u8>` | `None` |
| `on_start_day_change` | `Option<Callback<Option<u8>>>` | `None` |
| `end_day` | `Option<Signal<Option<u8>>>` | `None` |
| `default_end_day` | `Option<u8>` | `None` |
| `on_end_day_change` | `Option<Callback<Option<u8>>>` | `None` |
| `first_weekday` | `CalendarFirstWeekday` | 由 `DatePicker` 默认值决定 |
| `show_outside_days` | `bool` | `false` |
| `start_label` | `Option<String>` | `"Start"` |
| `end_label` | `Option<String>` | `"End"` |
| `start_placeholder` | `Option<String>` | `"Start date"` |
| `end_placeholder` | `Option<String>` | `"End date"` |
| `start_aria_label` | `Option<String>` | 回退到 `start_placeholder` |
| `end_aria_label` | `Option<String>` | 回退到 `end_placeholder` |
| `invalid_range_message` | `Option<String>` | `"End date must be on or after start date."` |
| `aria_label` | `Option<String>` | `"Date range picker"` |
| `class_name` | `Option<String>` | `None` |

### DateRangePicker Events

| Event | Type | Default |
| --- | --- | --- |
| `on_start_day_change` | `Callback<Option<u8>>` | `None` |
| `on_end_day_change` | `Callback<Option<u8>>` | `None` |

## Hello World（最小可用）

```rust
<DateRangePicker
  id_base="booking-range".to_string()
  start_year=2026
  start_month=6
  end_year=2026
  end_month=6
/>
```

- 默认路径无需用户手动管理受控状态。
- 需要业务控制时再接入 `start_day/end_day + on_*_change`。

## Semantics and Accessibility

- 根节点使用 `role="group"`，并暴露 `aria-label`。
- 输出稳定语义标记：`data-tone`、`data-state`、`data-has-start-value`、`data-has-end-value`、`data-has-full-value`、`data-partial`、`data-invalid-range`、`data-aria-source`、`data-class-source`。
- 非法区间时渲染 `data-slot="date-range-picker-hint"` 提示。

## Controlled / Uncontrolled Contract

- 起止日期轴都支持受控/非受控：
  - 受控：`start_day`/`end_day` + `on_start_day_change`/`on_end_day_change`
  - 非受控：`default_start_day`/`default_end_day`
- 默认值、文本回退、区间合法性在 `logic.rs` 统一归一，`view.rs` 仅消费结果。

## Motion and Fallback

- `DateRangePicker` 本体无独立 `motion.rs`。
- 动效由内部 `DatePicker` 的 motion 契约承担；在 non-wasm 路径可安全降级。

## Agent Contract / 流式降级

- 通过稳定 `data-*` 字段暴露状态和来源，供测试与 Agent 使用。
- 组件为表单输入部件，按 `Streaming Optional` 处理，默认 `snapshot` 渲染语义。

## docs-app Workbench（展示 / Config / Code / CSS Test）

- 展示区：展示“当前配置”并提供 `Valid` / `Invalid + Strong` 多场景对比。
- Config 区：可调 `start/end`、`tone`、`disabled`、自定义文案与自定义类。
- Code 区：实时生成与当前参数一致的复制片段。
- CSS Test 区：加载 `components/text-input/src/date_range_picker/styles.rs`，用于局部 CSS 契约验证。
