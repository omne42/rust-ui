# Command

`Command` 是命令检索组件，组合 `ui-headless::use_listbox` 与 `ActiveHighlightMotion`，用于在分组命令中进行键盘导航与筛选。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可筛选、可观测（稳定 `data-*`）的命令面板基元。
- 非目标：不负责弹层承载与打开关闭状态（由 `CommandDialog` / `Modal` 负责）。
- 风险边界：筛选逻辑、语义状态、样式契约必须同层维护，避免在 `view.rs` 临时补丁。

## Architecture Layers

- `logic.rs`：文本归一化、分组筛选、状态派生与 class 组合。
- `view.rs`：Leptos 结构渲染、键盘交互、headless listbox 绑定。
- `styles.rs`：槽位样式与 `data-state/data-*-source` 状态映射。
- `mod.rs`：对外导出 `Command`、`CommandGroup`、`CommandItem`、`CommandMotion` 与状态契约类型。

## Hello World

```rust
use std::sync::Arc;
use ui_components::{Command, CommandGroup, CommandItem};

let groups: Arc<[CommandGroup]> = Arc::from(vec![
    CommandGroup::new(
        "Navigation",
        vec![
            CommandItem::new("go-home", "Go Home"),
            CommandItem::new("open-settings", "Open Settings"),
        ],
    ),
]);

view! { <Command id_base="main-cmd".to_string() groups=groups /> }
```

## API 约定

- 必填：`id_base`、`groups`
- 可选：`on_action`、`disabled`、`motion`、`placeholder`、`empty_label`、`aria_label`、`class_name`
- 数据模型：
  - `CommandGroup { heading, items }`
  - `CommandItem { id, label, keywords, shortcut, disabled }`
- 查询匹配范围：`label` / `id` / `keywords`
- 语义观测：根节点输出 `data-state`、`data-items`、`data-groups`、`data-query`、`data-*-source`

## Keyboard / A11y

- 输入框角色：`combobox`，列表角色：`listbox`。
- 键盘：`ArrowUp/ArrowDown/Home/End/Enter` 委托给 listbox。
- 当查询非空且按 `Escape` 时，优先清空查询。

## Source-first

- 组件源码：`crates/ui-components/src/command/{mod,logic,view,styles}.rs`
- 组合组件：`crates/ui-components/src/command_dialog/`
- package feature：`component-command`（可选叠加 `inject-css`）

## Docs Playground

- docs-app `command` 页面已提供 `Interactive Playground`，包含展示区 + Config + Code + CSS Test。
- 同页保留多场景对比：分组搜索、自定义 placeholder/empty、状态与来源标记检查。
