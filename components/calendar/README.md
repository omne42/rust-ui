# Calendar

`Calendar` 是月视图日期网格组件，日期归一化与网格派生来自 `ui-state-primitives::calendar`。

## 快速开始（先用起来）

最小可用示例（Hello World）：

```rust
<Calendar
  year=2026
  month=3
/>
```

docs 入口：`#/components/calendar`

## 常见用法（基础）

- `Default + Outside Days`：`Sunday` 开始，显示外部月份日期
- `Monday First + Strong Tone`：`Monday` 开始 + `CalendarTone::Strong` + `is_show_outside_days=false`
- `Controlled vs Uncontrolled`：用 `default_selected_day`（非受控）或 `selected_day + on_selected_day_change`（受控）

核心对比例子：

```rust
<Calendar
  year=2026
  month=1
  selected_day=Some(6)
  tone=CalendarTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  is_show_outside_days=true
/>
<Calendar
  year=2026
  month=2
  selected_day=Some(14)
  tone=CalendarTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  is_show_outside_days=false
/>
```

## 进阶（需要时再看）

### 展示区（多场景对比）

docs-app 的 `Calendar` 页面提供三组对比：

- `Default + Outside Days`：`Sunday` 开始，显示外部月份日期
- `Monday First + Strong Tone`：`Monday` 开始 + `CalendarTone::Strong` + `is_show_outside_days=false`
- `Interactive Playground`：可切换月份、周起始、tone、outside days，并支持清空选中日

### Config 区

`Interactive Playground` 提供 settings 面板（Show settings）：

- `Month`：`Prev/Next`
- `Axes`：切换 `weekday`、`tone`、`outside days`
- `Clear selection`：清空 `selected_day`
- `config summary`：输出当前矩阵状态，便于比较不同组合

`selected_day` 轴支持受控/非受控成对 API：
- 受控：`selected_day + on_selected_day_change`
- 非受控：`default_selected_day + on_selected_day_change`
- 兼容：`on_day_press` 保留旧回调形态（`u8`）

### Code 区

每个 Playground 均支持 `Show code`，复制即用（自动补 import）。

### CSS Test 区

`Interactive Playground` 支持 `Show test`，可编辑 scoped CSS 并核对状态配置：

- CSS 来源：`components/calendar/src/styles.rs`
- `Actual config`：实时显示 `month/selected_day/tone/weekday/outside_days/class`
- 支持一键恢复原始样式

### Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()`
- 组件源码：`components/calendar/src/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/calendar.rs`
- package 模式前提：`component-calendar`（样式注入可选叠加 `inject-css`）
- 兼容迁移：旧参数 `show_outside_days` 仅保留兼容，优先使用 `is_show_outside_days`

### WASM 调试入口（feature gate）

- 组件级调试开关：`ui-calendar/wasm-debug`（默认关闭）。
- 聚合层转发开关：`ui-components` 的 `calendar-wasm-debug`（映射到 `ui-calendar/wasm-debug`）。
- 打开后，`Calendar` 根节点会渲染 `data-slot="calendar-debug"` 调试面板，提供：
  - 关键状态追踪：`trace_id/tick` + `prev_selected_day/next_selected_day` + `prev_source/next_source`
  - 关键交互回放：`data-action="replay-last-debug-event"` 按钮重放最近一次交互链路
- 默认构建与公共 API 不暴露调试入口，避免污染生产产物。
