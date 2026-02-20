# ContextMenu

`ContextMenu` 是一个基于 `ui-headless` + `Popover/Menu` + `ui-motion` 组合出来的右键/键盘上下文菜单组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的上下文菜单触发与弹层交互。
- 非目标：不在组件层承载业务状态管理、全局主题映射或通用动效引擎实现。
- 风险边界：如果状态机、A11y 或动效契约漂移，优先在对应层修复，不在 `view.rs` 追加临时补丁。

## Architecture Layers

- `logic.rs`：状态归一化与来源标记派生（id/aria/disabled/placement/open source）。
- `view.rs`：Leptos 结构渲染与交互挂载（右键、`Shift+F10`、`ContextMenu` 键、presence）。
- `motion.rs`：`ContextMenuMotion` 契约清洗与 wasm/non-wasm attach（non-wasm no-op）。
- `styles.rs`：静态 token-first CSS（稳定 `data-*`/class 选择器）。
- `mod.rs`：最小公开 API（`ContextMenu`、`ContextMenuMotion`、状态契约结构体与常量）。

## API (Table)

### ContextMenu Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `id_base` | `String` | `"context-menu"`（空白输入会归一为默认值） |
| `items` | `Vec<String>` | required |
| `on_action` | `Callback<usize>` | required |
| `is_disabled` | `Option<bool>` | `None`（优先于 `disabled`） |
| `disabled` | `bool` | `false`（兼容入口） |
| `disabled_indices` | `Vec<usize>` | `[]`（会去重并截断到 `item_count`） |
| `item_kinds` | `Vec<MenuItemKind>` | `[]` |
| `close_on_action` | `bool` | `true` |
| `placement` | `PopoverPlacement` | `PopoverPlacement::BottomStart` |
| `open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `motion` | `ContextMenuMotion` | `ContextMenuMotion::default()` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `aria_label` | `Option<String>` | `"Open context menu"` |
| `class_name` | `Option<String>` | `None` |

### ContextMenu Events

| Event | Type | Default |
| --- | --- | --- |
| `on_action` | `Callback<usize>` | required |
| `on_open_change` | `Callback<bool>` | `None` |

受控/非受控说明：
- 受控：传 `open`（可选配 `on_open_change`）。
- 非受控：传 `default_open` 初始化后由内部管理。

## Hello World（最小可用）

```rust
<ContextMenu
  id_base="demo-context-menu".to_string()
  items=vec!["Open".to_string(), "Rename".to_string(), "Delete".to_string()]
  on_action=Callback::new(move |_index: usize| {})
>
  "Right click or press Shift+F10"
</ContextMenu>
```

## Semantics and Accessibility

- 触发器暴露 `aria-haspopup="menu"`、`aria-expanded`、`aria-controls`。
- 菜单通过 `trigger_id/menu_id` 与 `aria_labelledby` 绑定语义关系。
- 支持右键 + 键盘打开：`ContextMenu` 键、`Shift+F10`、`ArrowDown`、`ArrowUp`。
- 支持 locale 接入：`lang`/`dir` 通过 `ui_headless::locale_attrs` 归一并挂载。
- 暴露稳定状态/来源标记：`data-state`、`data-open-mode`、`data-*-source`、`data-custom-*`。

## Motion and Fallback

- `ContextMenuMotion` 复用 `DropdownMenuMotion`，核心为 popover 动效契约。
- wasm 下走 popover motion attach；non-wasm 下为 no-op，保证 SSR/tooling 编译路径稳定。
- 组件侧只做语义到动效契约映射，不重写 spring/driver 执行器。

## Agent Contract / 流式降级

- 根节点输出机器可读字段：
  - `data-ui-schema="ui.context_menu.agent-contract.v1"`
  - `data-ui-schema-version="1"`
  - `data-ui-intent="open-context-actions"`
  - `data-ui-action`（`idle` / `open`）
  - `data-ui-state`（映射 `open/closed/disabled`）
  - `data-ui-source`（open 来源）
- 流式策略固定为快照降级：
  - `data-ui-stream-support="unsupported"`
  - `data-ui-stream-fallback="snapshot"`
  - `data-ui-stream-mode="snapshot"`
  - `data-ui-output-status`（`draft` / `submittable`）

## Feature Gate

- 组件特性：`component-context_menu`
- 依赖特性：`component-dropdown_menu`（由 `component-context_menu` 自动拉起）

## Tests and Docs

- 语义契约测试：`crates/ui-components/tests/context_menu_semantics.rs`
- docs-app 页面：`apps/docs-app/src/pages/components/pages/collections_command.rs` 中 `context_menu()`
