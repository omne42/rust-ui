# Menubar

`Menubar` is a desktop-style command bar component with keyboard navigation, controllable open state, and popover-backed menu content.

## Goals / Non-goals / Risk Boundary

- Goal: provide accessible menubar/menu trigger behavior with stable state/source markers.
- Non-goal: no business command dispatch policy or app routing orchestration in component layer.
- Risk boundary: keyboard/pointer behavior and open-index normalization must remain in logic/headless boundaries.

## Architecture Layers

- `logic.rs`: id/menu normalization, open-index sanitization, state/source attr derivation.
- `view.rs`: menubar rendering, keyboard/pointer handlers, presence/popover/menu composition.
- `styles.rs`: token-first static selectors for open mode/action mode/source markers.
- `mod.rs`: exports `Menubar`, `MenubarMenu`, slot/state contracts, and `MenubarMotion` alias.

Note: `MenubarMotion` is currently an alias to `DropdownMenuMotion`.

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | normalized; empty -> `DEFAULT_ID_BASE` |
| `menus` | `Vec<MenubarMenu>` | required |
| `on_action` | `Callback<(usize, usize)>` | required |
| `close_on_action` | `bool` | `DEFAULT_CLOSE_ON_ACTION` (`true`) |
| `placement` | `PopoverPlacement` | `DEFAULT_PLACEMENT` |
| `open_index` | `Option<Signal<Option<usize>>>` | `None` (controlled when provided) |
| `default_open_index` | `Option<usize>` | `None` |
| `on_open_index_change` | `Option<Callback<Option<usize>>>` | `None` |
| `motion` | `MenubarMotion` | `MenubarMotion::default()` |
| `class_name` | `Option<String>` | `None` |

`MenubarMenu` contract:
- `id`, `label`, `items` required via `MenubarMenu::new(...)`
- optional `disabled_indices(...)`, `item_kinds(...)`, `disabled(...)`

## Hello World

```rust
<Menubar
  id_base="app-menubar".to_string()
  menus=menus
  on_action=on_action
/>
```

## Docs Playground（展示区）

### 展示 (Display)

- workbench 预览不同菜单集、开关行为、placement 和 motion 的组合效果。
- 展示实时 `open menu index` 与 `last action`。

### config

- `Menu set`：`app / workspace / compact`。
- `Close on action`、`Flip placement`、`Default open menu`。
- `Custom class`、`Custom motion`。

### code

- `code` 面板输出当前 workbench 配置对应的 `Menubar` 代码。
- 非默认配置会展开显式 prop，便于回归复现。

### css test

- `css test` 面板绑定 `crates/ui-components/src/menu/menubar/styles.rs`。
- 可验证 open-mode、placement、motion-source 相关样式契约。

### 多场景对比显示

- `Desktop Menubar + Action Dispatch`、`Controlled Open + Persistent + Disabled Menu`、`State + Source Markers` 三组并行场景对比。

## Semantics and Accessibility

- Root uses `role="menubar"` and triggers use `role="menuitem"`.
- Keyboard behavior includes `ArrowLeft`/`ArrowRight` navigation and `ArrowDown`/`ArrowUp` open focus strategy.
- Stable source markers include `data-open-mode`, `data-id-source`, `data-open-index-source`, `data-motion-source`, and related `data-custom-*` flags.

## Motion and Fallback

- Menu content composes `Popover` + `Menu` + `use_presence` for motion-safe enter/exit.
- Motion contract is provided through `MenubarMotion` (dropdown-menu motion alias).
- Non-wasm remains compile-safe through dependency-layer fallbacks.

## Source-first / Copy-Paste Ready

- Docs entry: `apps/docs-app/src/pages/components/pages/collections_command.rs::menubar()`
- Source: `crates/ui-components/src/menu/menubar/{mod,logic,view,styles}.rs`
- Package mode feature: `component-menubar` (practically composed with `component-menu`, `component-popover`, `component-dropdown_menu`; optional `inject-css`)
