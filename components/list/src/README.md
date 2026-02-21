# List

`List` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion` 组合出来的列表选择组件，包含 `List`、`ListItem`、`ListSection` 三个装配层入口。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可观测、可测试的列表导航与选择语义契约。
- 非目标：不在组件层绑定业务 store，不在 `view.rs` 重写跨组件复用状态机。
- 风险边界：若选择状态、键盘语义或动效执行漂移，优先回迁 primitives/headless/motion 层修复。

## Architecture Layers

- `logic.rs`：列表/条目/分组状态归一与派生（`ListState`、`ListItemState`、`ListSectionState`）。
- `view.rs`：Leptos 结构渲染与 headless hooks 挂载（`use_listbox`、`use_focus_ring`）。
- `motion.rs`：`ListMotion`/`ListSectionMotion` 动效契约与参数清洗。
- `styles.rs`：静态 CSS 契约，仅依赖稳定 `data-*` / class 选择器。
- `mod.rs`：最小导出面（`List`、`ListItem`、`ListSection`、`ListMotion`、`ListSectionMotion`、状态枚举）。
- `ui-state-primitives/src/list.rs`：纯 list state 原语（键/索引选择与前后移动）。

## API (Table)

### List Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `Option<String>` | `None`（优先消费 `UiRoot` 注入的 `IdProvider`，否则回退 `ui-list`） |
| `items` | `Arc<[String]>` | required |
| `selected_index` | `Option<Signal<Option<usize>>>` | `None` |
| `default_selected_index` | `Option<usize>` | `None` |
| `on_selected_index_change` | `Option<Callback<Option<usize>>>` | `None` |
| `id` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None`（内部 fallback） |
| `aria_labelledby` | `Option<String>` | `None` |
| `is_disabled` | `bool` | `false` |
| `disabled_indices` | `Vec<usize>` | `[]` |
| `on_action` | `Option<Callback<usize>>` | `None` |
| `default_active_index` | `usize` | `0` |
| `is_active_index_synced_to_selected` | `bool` | `true` |
| `motion` | `ListMotion` | `ListMotion::default()` |
| `class_name` | `Option<String>` | `None` |

### ListItem Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `id` | `Option<String>` | `None` |
| `index` | `Option<usize>` | `None` |
| `is_selected` | `bool` | `false` |
| `is_focused` | `bool` | `false` |
| `is_disabled` | `bool` | `false` |
| `is_selection_indicator_visible` | `bool` | `false` |
| `is_divider_visible` | `bool` | `false` |
| `aria_label` | `Option<String>` | `None`（内部 fallback） |
| `on_press` | `Option<Callback<()>>` | `None` |
| `on_pointer_move` | `Option<Callback<()>>` | `None` |
| `class_name` | `Option<String>` | `None` |

### ListSection Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `title` | `Option<String>` | `None` |
| `item_count` | `Option<usize>` | `Some(1)` |
| `heading_tone` | `ListSectionHeadingTone` | `Default` |
| `is_disabled` | `bool` | `false` |
| `is_sticky_heading` | `bool` | `false` |
| `is_divider_visible` | `bool` | `false` |
| `motion` | `ListSectionMotion` | `ListSectionMotion::default()` |
| `aria_label` | `Option<String>` | `None`（内部 fallback） |
| `class_name` | `Option<String>` | `None` |

## Controlled / Uncontrolled 语义

- `List` 选择轴统一为三件套：`selected_index + on_selected_index_change + default_selected_index`。
- 受控模式：提供 `selected_index` 时，外部值为单一事实来源，组件仅通过 `on_selected_index_change` 发出变更请求。
- 非受控模式：未提供 `selected_index` 时，仅使用 `default_selected_index` 初始化一次，后续由组件内部状态原语管理。
- `default_active_index` 仅作用于 active roving 焦点，不作为选中值事实来源。
- `is_active_index_synced_to_selected` 控制 active 索引是否跟随当前选中值。

## Streaming 策略

- `Snapshot`：默认路径，组件稳定消费完整配置并渲染。
- `Streaming Optional`：`List` 不是 LLM 正文阅读面；若上层为流式容器，本组件按 `fallback=snapshot` 消费稳定配置。

## Hello World

```rust
use leptos::prelude::*;
use std::sync::Arc;
use ui_components::List;

let items: Arc<[String]> = vec!["Overview".to_string(), "Billing".to_string()].into();
view! { <List id_base="list-hello".to_string() items=items aria_label="Settings navigation".to_string() /> }
```

## 展示 (Display)

- docs-app 页面：`apps/docs-app/src/pages/components/pages/collections.rs` 的 `list()`。
- 展示区包含多场景对比：
  - 默认 + 禁用项（`disabled_indices`）
  - active/selected 解耦（`is_active_index_synced_to_selected=false`）
  - 根禁用（`is_disabled=true`）
  - 空列表（`items=Vec::<String>::new().into()`）

## 状态矩阵（受控 / 非受控）

- docs-app `list()` 新增 `状态矩阵 State Matrix（受控 / 非受控）` Playground。
- 同一组数据固定对照 `uncontrolled`、`controlled`、`disabled` 三个状态轴。
- 受控路径通过 `selected_index + on_selected_index_change` 显式联动，便于回归验证。

## Config (Workbench Settings)

- Workbench 使用统一 `Playground controls` 面板调节：
  - `Sync active index to selected`
  - `Disable last option`
  - `Disable root`
  - `Custom class marker`
- 重点是让语义状态（selection/disabled/data-*）在同一画布连续观察，降低回归定位成本。

## Code (Workbench Snippet)

- Workbench 通过 `code_signal` 输出当前配置对应的最小可复制代码。
- 开关变化会同步体现在代码中（如 `is_active_index_synced_to_selected`、`is_disabled`、`disabled_indices`、`class_name`）。

## CSS Test (Scoped CSS)

- Workbench 通过 `test_css_source` 加载三段样式契约：
  - `components/list/src/styles.rs::CSS`
  - `components/list/src/styles.rs::ITEM_CSS`
  - `components/list/src/styles.rs::SECTION_CSS`
- CSS Test 面板用于局部覆盖与回放，`test_config_signal` 同步输出实际配置，保证样式测试可追溯。

## Streaming/Snapshot Display

- docs-app `list()` 提供 `Streaming/Snapshot Display` Playground。
- `List` 在文档中按 `Streaming Optional` 展示，明确 `fallback=snapshot`。
- 双列同时挂载 `data-ui-streaming="optional"` 与 `data-ui-fallback="snapshot"`，分别展示 `snapshot`/`streaming` 输出状态。

## Source-first / Copy-Paste Ready

- 所有 Playground 的 `Show code` 都走 `apps/docs-app/src/playground.rs::compose_copy_ready_code`，复制时自动补齐缺失 imports。
- 页面包含 `Copy starter` 按钮与真实源码落点：`components/list/src/{mod,logic,view,styles,motion}.rs`。
- 依赖前提固定为 `component-list` + `inject-css`，避免复制后缺 feature 报错。

## Semantics and Accessibility

- 根节点语义：`role="listbox"`，条目语义：`role="option"`。
- 选中/焦点/禁用状态通过 `aria-*` 与稳定 `data-*` 同步暴露。
- 关键状态标记包含：`data-empty`、`data-has-selection`、`data-disabled`、`data-motion-source` 等。
- 分组语义由 `ListSection` 提供 `role="group"` 与标题/tone/source 标记。

## Motion and Fallback

- `List` 使用 active-highlight 动效契约（`ListMotion`）。
- `ListSection` 使用 illustrated-message 动效契约（`ListSectionMotion`）。
- 参数会先经过 `motion.rs` 清洗，non-wasm 走 no-op/stub 路径保证编译稳定。

## Testing Contract

- 模块/文档契约测试：`crates/ui-components/tests/list_module_semantics.rs`
- docs 页面示例：`apps/docs-app/src/pages/components/pages/collections.rs`、`apps/docs-app/src/pages/components/pages/collections_extra.rs`
- 组件页 E2E 覆盖：`e2e/tests/docs_app_components_coverage.spec.mjs`

## Source-first

- 组件源码：`components/list/src/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/list.rs`
