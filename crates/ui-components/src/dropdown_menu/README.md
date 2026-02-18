# DropdownMenu

`DropdownMenu` 是一个由 `Button + Popover + Menu` 组合的菜单触发组件，提供统一的打开策略、受控/非受控 open 语义和稳定状态标记。

## 目标 / 非目标 / 风险边界

- 目标：在最小 API 下提供可访问的菜单触发与动作分发能力。
- 非目标：不在本组件层实现菜单项渲染协议扩展（复杂项由 `Menu` 负责）。
- 风险边界：open 状态契约统一走 controllable state；组件层不重写 headless overlay 语义。

## Architecture Layers

- `logic.rs`：`id`、禁用索引、打开焦点策略与根状态派生。
- `motion.rs`：`DropdownMenuMotion` 到 `PopoverMotion` 的契约映射与清洗。
- `view.rs`：组合 `Button`/`Popover`/`Menu`，挂载 `aria-*` 与 `data-*`。
- `styles.rs`：组件级静态 CSS。
- `mod.rs`：导出 `DropdownMenu` 与 `DropdownMenuMotion`。

## API (Table)

### DropdownMenu Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required（空值回退 `dropdown-menu`） |
| `items` | `Vec<String>` | required |
| `on_action` | `Callback<usize>` | required |
| `disabled` | `bool` | `false` |
| `disabled_indices` | `Vec<usize>` | `[]`（自动去重并按项数裁剪） |
| `item_kinds` | `Vec<MenuItemKind>` | `[]` |
| `close_on_action` | `bool` | `true` |
| `placement` | `PopoverPlacement` | `PopoverPlacement::default()` |
| `open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `trigger_variant` | `ButtonVariant` | `ButtonVariant::Secondary` |
| `trigger_size` | `ButtonSize` | `ButtonSize::Sm` |
| `motion` | `DropdownMenuMotion` | `DropdownMenuMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `children` | `Children`（trigger 内容） | required |

### DropdownMenu Events

| Event | Type | Default |
| --- | --- | --- |
| `on_action` | `Callback<usize>` | required |
| `on_open_change` | `Callback<bool>` | `None` |

## Hello World（最小可用）

```rust
view! {
    <DropdownMenu
        id_base="docs-dropdown".to_string()
        items=vec!["Edit".to_string(), "Archive".to_string()]
        on_action=Callback::new(|index| logging::log!("action: {}", index))
    >
        "Actions"
    </DropdownMenu>
}
```

## Controlled / Uncontrolled

- 非受控：只传 `default_open`（或都不传）由内部状态管理。
- 受控：传 `open` + `on_open_change`，外部值为单一事实来源。
- 根节点会输出 `data-controlled` / `data-uncontrolled` 便于测试与排障。

## Semantics and Accessibility

- 触发器使用 `aria-haspopup="menu"`、`aria-expanded`、`aria-controls`。
- 支持键盘打开策略：`ArrowDown` 聚焦首项，`ArrowUp` 聚焦末项。
- 稳定语义标记：`data-state`、`data-placement`、`data-close-on-action`、`data-has-disabled-items`、`data-has-item-kinds`、`data-motion-source`。

## Motion and Fallback

- 组件动效契约只包装 `PopoverMotion`，默认值与清洗逻辑复用 `Popover`。
- 非法动效参数会在 `sanitize_motion` 中归一化，不在 `view.rs` 分散兜底。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：预览触发器、弹层和菜单行为（包含 open/close 与 action 反馈）。
- `Config`：可切换 item 数量、close 策略、controlled 模式、disabled 索引、class/motion 来源。
- `Code`：实时输出当前参数组合对应的 `DropdownMenu` 示例代码。
- `CSS Test`：加载 `dropdown_menu/styles.rs` 原始样式，支持 scoped 覆盖验证。
- `对比`：页面保留 `Default / Controlled + Persistent / Disabled + Empty` 多场景对比。
