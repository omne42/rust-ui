# Accordion

`Accordion` 是一个基于 `ui-core` + `ui-headless` + `ui-motion` 组合出来的折叠面板组件。

## 架构分层

- `logic.rs`：纯状态归一化与派生（`AccordionSelectionMode`、open 集合切换、根状态派生）。
- `view.rs`：Leptos 结构渲染与 headless hooks 挂载（press/focus/hover/roving）。
- `motion.rs`：`AccordionMotion` 契约与 wasm 动效驱动，含 SSR no-op 与 reduced-motion 降级。
- `styles.rs`：仅静态 CSS 契约，样式由 `var(--ui-*)` 驱动。
- `mod.rs`：公开最小稳定 API（`Accordion`、`AccordionSelectionMode`、`AccordionMotion`）。

## API 概览

- `labels: Vec<String>`：每个触发器的显示文本（由调用方提供，支持 i18n）。
- `id_base: String`：生成触发器/面板的稳定 id。
- `open_indices: Option<Signal<BTreeSet<usize>>>`：受控打开状态。
- `default_open_indices: Option<BTreeSet<usize>>`：非受控初始状态。
- `on_open_change: Option<Callback<BTreeSet<usize>>>`：状态变更回调。
- `selection_mode: AccordionSelectionMode`：`Single` / `Multiple`。
- `disabled: bool`：全局禁用。
- `disabled_indices: Vec<usize>`：按索引禁用。
- `motion: AccordionMotion`：组件级动效参数。
- `class_name: Option<String>`：附加类名。

## 语义与可访问性

- 触发器与面板通过 `id` + `aria-controls` / `aria-labelledby` 绑定。
- 面板使用 `role="region"`，触发器暴露 `aria-expanded`。
- 使用 roving tabindex 支持方向键焦点导航。
- 暴露稳定 `data-*` 状态标记用于样式与测试契约。

## 动效与降级

- wasm 下使用 `ui-motion::spring::SpringAnimator`。
- 非 wasm 环境采用 no-op，保证 SSR 编译通过。
- 检测 `prefers-reduced-motion: reduce` 时禁用弹簧过渡，直接应用最终状态。
