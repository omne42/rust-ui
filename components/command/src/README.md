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
- `motion.rs`：动效参数归一与高亮动效挂载。
- `mod.rs`：对外导出 `Command`、`CommandGroup`、`CommandItem`、`CommandMotion` 与状态契约类型。

## Hello World

```rust
use std::sync::Arc;
use ui_components::{Command, CommandGroup, CommandItem};

let groups: Arc<[CommandGroup]> = Arc::from(vec![CommandGroup::new(
    "Navigation",
    vec![CommandItem::new("go-home", "Go Home")],
)]);

view! { <Command id_base="main-cmd".to_string() groups=groups /> }
```

## 常见用法

### 受控查询（外部状态驱动）

```rust
let (query, set_query) = signal("cal".to_string());

view! {
  <Command
    id_base="main-cmd".to_string()
    groups=groups.clone()
    query=Some(Signal::derive(move || query.get()))
    on_query_change=Some(Callback::new(move |next: String| set_query.set(next)))
  />
}
```

### 非受控查询（默认值初始化）

```rust
view! {
  <Command
    id_base="main-cmd".to_string()
    groups=groups
    default_query=Some("cal".to_string())
  />
}
```

## 新手路径（先用起来，再进阶）

- 第一步（默认 API）：先用 `<Command id_base=... groups=... />` 跑通交互。
- 第二步（常见定制）：按需启用 `default_query` 或 `on_action`。
- 第三步（进阶控制）：需要外部单一事实源时再接入 `query + on_query_change`。
- 第四步（高级覆盖）：只在必要时覆盖 `placeholder/empty_label/aria_label/class_name/motion`。

## API 约定

- 必填：`id_base`、`groups`
- 可选：`query`、`default_query`、`on_query_change`、`on_action`、`is_disabled`、`motion`、`placeholder`、`empty_label`、`aria_label`、`lang`、`dir`、`class_name`
- 命名迁移：`disabled` 已统一为 `is_disabled`；示例与文档统一使用新命名，避免同义别名漂移。
- 查询轴采用受控/非受控成对接口：`query + on_query_change + default_query`。
- 数据模型：
  - `CommandGroup { heading, items }`
  - `CommandItem { id, label, keywords, shortcut, disabled }`
- 组合语义：配置输入只接受类型化 `CommandGroup + CommandItem` 语义树；不使用 `labels + children` / `titles + panels` 并行数组约定。
- 查询匹配范围：`label` / `id` / `keywords`
- 语义观测：根节点输出 `data-state`、`data-items`、`data-groups`、`data-query`、`data-*-source`

## Keyboard / A11y

- 输入框角色：`combobox`，列表角色：`listbox`。
- 键盘：`ArrowUp/ArrowDown/Home/End/Enter` 委托给 listbox。
- 当查询非空且按 `Escape` 时，优先清空查询。

## Source-first

- 组件源码：`components/command/src/{mod,logic,view,styles,motion}.rs`
- 组合组件：`crates/ui-components/src/command_dialog/`
- package feature：`component-command`（可选叠加 `inject-css`）

## Docs Playground

- docs-app `command` 页面已提供 `Interactive Playground`，包含展示区 + Config + Code + CSS Test。
- 同页保留多场景对比：分组搜索、自定义 placeholder/empty、状态与来源标记检查。
