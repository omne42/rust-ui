# Calendar

`Calendar` 是月视图日期网格组件，日期归一化与网格派生来自 `ui-state-primitives::calendar`。

## 展示区（多场景对比）

docs-app 的 `Calendar` 页面提供三组对比：

- `Default + Outside Days`：`Sunday` 开始，显示外部月份日期
- `Monday First + Strong Tone`：`Monday` 开始 + `CalendarTone::Strong` + `show_outside_days=false`
- `Interactive Playground`：可切换月份、周起始、tone、outside days，并支持清空选中日

## Config 区

`Interactive Playground` 提供 settings 面板（Show settings）：

- `Month`：`Prev/Next`
- `Axes`：切换 `weekday`、`tone`、`outside days`
- `Clear selection`：清空 `selected_day`
- `config summary`：输出当前矩阵状态，便于比较不同组合

## Code 区

每个 Playground 均支持 `Show code`，复制即用（自动补 import）。

核心对比例子：

```rust
<Calendar
  year=2026
  month=1
  selected_day=Some(6)
  tone=CalendarTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>
<Calendar
  year=2026
  month=2
  selected_day=Some(14)
  tone=CalendarTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
/>
```

## CSS Test 区

`Interactive Playground` 支持 `Show test`，可编辑 scoped CSS 并核对状态配置：

- CSS 来源：`crates/ui-components/src/calendar/styles.rs`
- `Actual config`：实时显示 `month/selected_day/tone/weekday/outside_days/class`
- 支持一键恢复原始样式

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()`
- 组件源码：`crates/ui-components/src/calendar/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-logic-calendar/src/calendar.rs`
- package 模式前提：`component-calendar`（样式注入可选叠加 `inject-css`）
