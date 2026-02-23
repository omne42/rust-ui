# Pagination

`Pagination` 是一个基于 `ui-state-primitives` 状态归一 + `ui-headless` 按压语义 + `ui-theme` 样式契约组装的分页组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可观测、可测试的分页导航基元。
- 非目标：不在组件层承载业务数据加载、路由同步或全局状态管理。
- 风险边界：状态派生规则必须收敛在 `logic.rs`，`view.rs` 只做渲染与事件挂载。

## Architecture Layers

- `logic.rs`：页码状态归一（clamp/empty/prev-next disabled）、分页范围计算、可选文本归一。
- `view.rs`：Leptos 结构渲染，挂载 `OnPress` 交互与 i18n 文案。
- `styles.rs`：静态样式契约，仅消费 `var(--ui-*)`。
- `i18n.rs`：`PaginationStrings` 默认文案（`aria_label`、上一页/下一页标签）。
- `mod.rs`：最小公开导出（`Pagination`、`PaginationStrings`、`PaginationItem`、`resolve_pagination_range`）。

说明：分页当前不包含独立 `motion.rs`，默认走无动画快照渲染路径。

## Hello World（最小可用）

```rust
let (page, set_page) = signal(1_usize);
view! { <Pagination total_pages=12 page=page set_page=set_page /> }
```

## 展示区（docs-app）

- `展示 Display`：基础分页 + `on_change` 行为展示。
- `Config 配置对比`：`siblings/boundaries` 不同配置并排对比。
- `Code 代码示例`：最小可复制片段与实际运行结果对照。
- `CSS Test`：`class_name="docs-pagination-custom"` 自定义样式契约验证。
- `状态对比 State Matrix`：`first/middle/last/disabled/empty` 多场景并排对比。

对应文档入口：`apps/docs-app/src/pages/components/pages/collections.rs::pagination()`。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `total_pages` | `usize` | required |
| `page` | `ReadSignal<usize>` | required |
| `set_page` | `WriteSignal<usize>` | required |
| `siblings` | `usize` | `0` |
| `boundaries` | `usize` | `0` |
| `disabled` | `bool` | `false` |
| `on_change` | `Option<Callback<usize>>` | `None` |
| `aria_label` | `Option<String>` | `None`（回退到 i18n 默认文案） |
| `class_name` | `Option<String>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `on_change` | `Callback<usize>` | `None` |

## Semantics and Accessibility

- 根节点是 `nav`，带 `aria-label`，并暴露稳定状态标记：`data-slot="pagination"`、`data-page`、`data-total-pages`、`data-empty`、`data-disabled`、`data-single-page`。
- 上一页/下一页节点分别暴露 `data-slot="pagination-prev"` / `data-slot="pagination-next"` 和 `data-disabled`。
- 页码节点暴露 `data-slot="pagination-page"`、`data-page`、`data-current`；当前页设置 `aria-current="page"`。
- 省略节点暴露 `data-slot="pagination-dots"` 与 `data-slot="pagination-dots-label"`。
- 用户可见文案通过 `ui_headless::i18n::use_ui_i18n()` + `PaginationStrings` 注入，不在 `view.rs` 硬编码。

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/collections.rs::pagination()`
- 组件源码：`components/pagination/src/{mod,logic,view,styles,i18n}.rs`
- 语义回归：`components/pagination/test/semantics.rs`
- e2e 合约：`e2e/tests/docs_app_pagination_contract.spec.mjs`
- package 模式最小特性：`component-pagination`（会联动 `component-button`）；样式注入可叠加 `inject-css`。
