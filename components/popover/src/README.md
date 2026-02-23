# Popover

`Popover` 是一个基于 anchor 定位的 overlay 组件，支持 modal/non-modal 语义、退出回调与可定制 spring 动效。

## 目标 / 非目标 / 风险边界

- 目标：提供稳定的 open/close + placement + modal 契约，以及可测试的 source markers。
- 非目标：不在组件层承载业务异步流程与流式正文协议。
- 风险边界：Escape 关闭、焦点陷阱、overlay 栈规则应保持在既有 headless 契约路径，避免在 `view.rs` 打补丁。

## Architecture Layers

- `logic.rs`：slot/状态/来源标记派生，class 组装，escape 关闭条件判定。
- `view.rs`：Portal 渲染、headless hooks 挂载（modal/focus-trap/position/overlay-stack）。
- `motion.rs`：`PopoverMotion` 契约、sanitize、wasm spring attach、non-wasm no-op。
- `styles.rs`：token-first 静态 CSS 与 state/source selector。
- `mod.rs`：公开 API（`Popover`、`PopoverMotion`、`PopoverSlot`、`PopoverPartState*`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `is_open` | `Option<Signal<bool>>` | `None` |
| `open`（兼容别名） | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None`（归一为 `false`） |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `on_close`（兼容别名） | `Option<OnPress>` | `None` |
| `anchor_ref` | `NodeRef<html::Button>` | required |
| `children` | `ChildrenFn` | required |
| `placement` | `PopoverPlacement` | `PopoverPlacement::default()` |
| `motion` | `PopoverMotion` | `PopoverMotion::default()` |
| `is_modal` | `bool` | `true` |
| `aria_labelledby` | `Option<String>` | `None` |
| `aria_describedby` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |

## Hello World（最小可用）

```rust
let anchor_ref = NodeRef::<html::Button>::new();
<button node_ref=anchor_ref>"Anchor"</button>
<Popover anchor_ref=anchor_ref default_open=true>
  {move || view! { <div>"Popover content"</div> }}
</Popover>
```

- 默认路径：`anchor_ref + children` 即可跑通（可选 `default_open`）。
- 高级路径：按需叠加 `is_open/default_open/on_open_change`、`motion`、`is_modal`。

## Slot Projection（Lazy / KeepAlive / Eager）

- `Lazy`：调用方通过 `<Show when=present>` 控制挂载，通常与 `on_exit_complete` 配合，在退出动画完成后再卸载。
- `KeepAlive`：保持 `Popover` 挂载，只切换 `open`；关闭时会触发 `on_exit_complete`，用于暂停/收敛高耗能副作用。
- `Eager`：始终挂载 `<Popover ...>`，不做 presence 条件卸载；内容持续存在，由 `data-state` 与 motion 驱动可见性。

## 命名兼容与迁移

- 规范命名：`is_open / default_open / on_open_change`。
- 兼容别名：`open / on_close` 仍可用，统一在 `logic::normalize_open_state` 归一。
- 优先级：`is_open > open`；`on_open_change > on_close`。
- 迁移建议：先替换 `open -> is_open` 与 `on_close -> on_open_change`，再移除别名调用。

## Semantics and Accessibility

- 根节点标记：`data-state`、`data-modal`、`data-placement`。
- 来源标记：`data-motion-source`、`data-placement-source`、`data-modal-source`、`data-class-source`、`data-exit-source`、`data-open-mode`、`data-open-state-source`、`data-open-source`、`data-default-open-source`、`data-open-change-source`、`data-dismiss-source`。
- 封闭集合：`data-open-state-source ∈ {external, default, implicit-default}`；`data-dismiss-source ∈ {none, outside-press, escape-key}`。
- Panel A11y：`role="dialog"`、`aria-modal`、`aria-labelledby`、`aria-describedby`，并支持 `lang/dir` 透传。
- Escape 关闭需同时满足：topmost、非输入法 composing、`defaultPrevented=false`。
- modal 模式挂载 `use_modal` 与焦点陷阱；panel 支持 `tabindex=-1` 键盘接管。

## Motion and Fallback

- 默认 spring：`stiffness=300`、`damping=25`、`mass=1`、`initial_scale=0.98`、`offset_y_px=6`。
- wasm：使用 `ui_motion::spring::SpringAnimator` 驱动 `opacity/scale/y`。
- non-wasm：关闭时触发 no-op 完成回调，保证 SSR/tooling 可编译。
- `sanitize_motion` 对 spring/scale/offset 做回落与 clamp。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/overlays.rs`
- 页面：`popover()`
- Playground：`Popover`、`State + Source Markers`、`Workbench (Display + Config + Code + CSS Test)`

## Playground 展示区（Display / Config / Code / CSS Test）

- 展示（Display）：实时验证 `open/modal/motion/class` 的组合行为与关闭路径。
- 配置（Config）：Workbench 控件调节 `initial_scale/offset/modal/class`，并输出 `PopoverWorkbenchConfig`。
- 代码（Code）：根据当前设置生成 `Popover` 代码片段，含 motion/custom flags。
- CSS Test：加载 `popover/styles.rs`，在 scoped 测试面板里直接调试状态选择器。

## 多场景对比展示

- `Popover`：默认契约路径（open/close + anchor 定位 + exit complete）。
- `State + Source Markers`：自定义 motion + non-modal + custom class 的来源标记对比。
- `Workbench`：同画布切换 modal/non-modal、motion 强度、custom class，观察配置差异。

## Source-first

- `components/popover/src/mod.rs`
- `components/popover/src/logic.rs`
- `components/popover/src/view.rs`
- `components/popover/src/motion.rs`
- `components/popover/src/styles.rs`
