# Dropdown

`Dropdown` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion` 组合出来的菜单触发组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试、可观测的下拉菜单触发交互基元。
- 非目标：不在组件层实现业务 store 绑定、不承载主题系统定义、不重写跨组件交互原语。
- 风险边界：若状态机、A11y 语义或动效执行出现分层漂移，优先回迁到 primitives/headless/motion 对应层修复，不在 `view.rs` 打补丁。

## Architecture Layers

- `logic.rs`：输入归一与来源标记装配；组件级 alias 兼容（`is_*` + legacy）与 class 组合。
- `view.rs`：Leptos 结构渲染与 headless 契约挂载（controllable open state + presence + aria）。
- `motion.rs`：`DropdownMotion` 契约与 popover 动效参数清洗，保证 wasm/SSR 路径一致。
- `styles.rs`：仅静态 CSS 契约；状态样式依赖稳定 `data-*` / class 标记。
- `mod.rs`：最小稳定导出面（`Dropdown`、`DropdownMotion`）。
- `ui-state-primitives/src/dropdown.rs`：纯状态原语（open focus strategy、输入归一、状态派生）。

## API (Table)

### Dropdown Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `id_base` | `String` | required |
| `items` | `Vec<String>` | required |
| `on_action` | `Callback<usize>` | required |
| `is_disabled` | `Option<bool>` | `None` |
| `disabled` | `bool` (legacy alias) | `false` |
| `disabled_indices` | `Vec<usize>` | `[]` |
| `item_kinds` | `Vec<MenuItemKind>` | `[]` |
| `close_on_action` | `bool` | `true` |
| `placement` | `PopoverPlacement` | `PopoverPlacement::BottomStart`（headless 默认） |
| `is_open` | `Option<Signal<bool>>` | `None` |
| `open` | `Option<Signal<bool>>` (legacy alias) | `None` |
| `default_open` | `Option<bool>` | `None` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `motion` | `DropdownMotion` | `DropdownMotion::default()` |
| `aria_label` | `Option<String>` | `None`（内部 fallback） |
| `class_name` | `Option<String>` | `None` |

### Dropdown Events

| Event | Type | Default |
| --- | --- | --- |
| `on_action` | `Callback<usize>` | required |
| `on_open_change` | `Callback<bool>` | `None` |

## Controlled / Uncontrolled 契约

- open 轴遵循 triplet：`is_open/open` + `on_open_change` + `default_open`。
- `is_open` 与 `open` 同时提供时，优先 `is_open`。
- 组件内部通过 `ui_headless::use_controllable_open_state_traced("dropdown", ...)` 统一受控/非受控语义。

## Streaming 策略

- `Snapshot`：默认路径，组件稳定消费完整配置并渲染。
- `Streaming Optional`：`Dropdown` 不是 LLM 正文阅读面；若上层为流式容器，本组件按 `fallback=snapshot` 消费稳定配置。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::Dropdown;

let on_action = Callback::new(move |_: usize| {});

view! {
    <Dropdown
        id_base="profile-dropdown".to_string()
        items=vec!["Profile".to_string(), "Settings".to_string()]
        on_action=on_action
    >
        "Open actions"
    </Dropdown>
}
```

## 受控 open 示例

```rust
use leptos::prelude::*;
use ui_components::Dropdown;

let (open_raw, set_open_raw) = signal(false);
let open = Signal::derive(move || open_raw.get());
let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));
let on_action = Callback::new(move |_: usize| {});

view! {
    <Dropdown
        id_base="profile-dropdown-controlled".to_string()
        items=vec!["Rename".to_string(), "Duplicate".to_string(), "Archive".to_string()]
        on_action=on_action
        is_open=open
        on_open_change=on_open_change
        close_on_action=false
    >
        "Open actions"
    </Dropdown>
}
```

## docs-app 展示区（展示 / config / code / css test）

`Dropdown` 页面已提供与 `button` 同类的交互式 Playground，包含四个可切换展示区：

- 展示区（Display）：实时渲染组件交互结果。
- Config 区：通过控件切换 `controlled`、`placement`、`close_on_action`、`disabled`、`motion` 等配置。
- Code 区：展示当前配置对应的可复制代码片段。
- CSS Test 区：加载 `dropdown/styles.rs` 的原始样式并支持局部覆盖，同时展示 `Actual config`。

对应入口：

- `Interactive Playground`：`apps/docs-app/src/pages/components/pages/collections_extra.rs`
- `test_css_source` 来源：`crates/ui-components/src/menu/dropdown/styles.rs`

## 多场景对比展示

页面提供 `State Matrix Compare` 对比区，覆盖至少四种状态：

- `Default`
- `Controlled + Persistent`
- `Disabled`
- `Empty`

用于快速验证受控/非受控、禁用态、空数据态在同一视图下的语义与样式差异。

## Semantics and Accessibility

- 触发器语义：`aria-haspopup="menu"`、`aria-expanded`、`aria-controls` 与 menu id 绑定。
- 键盘语义：关闭态支持 `ArrowDown/ArrowUp` 打开并设置 focus strategy。
- 国际化接入：支持 `lang` / `dir` 透传，不硬编码方向假设。
- 可观测契约：根节点暴露 `data-state`、`data-open`、`data-closed`、`data-controlled`、`data-aria-source`、`data-class-source` 等稳定字段。

## Motion and Fallback

- popover 动效通过 `DropdownMotion { popover: PopoverMotion }` 配置。
- `sanitize_motion` 在视图层挂载前统一清洗参数，避免无效值泄漏。
- non-wasm 路径依赖 `ui-motion` no-op/stub，保证 SSR/tooling 可编译。
- reduced-motion 由底层动效能力统一降级处理。

## Testing Contract

- 语义契约测试：`crates/ui-components/tests/dropdown_semantics.rs`
- docs-app 页面示例：`apps/docs-app/src/pages/components/pages/collections_extra.rs`
- E2E 合同测试：`e2e/tests/docs_app_dropdown_contract.spec.mjs`

## Agent Contract / AI 可读性

- 关键状态轴以机器可读语义暴露（`data-*` + source markers），减少自动化对 DOM 结构猜测依赖。
- 输出模式按组件职责标识为流式可选，默认走快照渲染路径。

## Source-first

- 组件源码：`crates/ui-components/src/menu/dropdown/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/dropdown.rs`
