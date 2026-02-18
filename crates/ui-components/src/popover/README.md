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
| `open` | `Signal<bool>` | required |
| `anchor_ref` | `NodeRef<html::Button>` | required |
| `on_close` | `OnPress` | required |
| `children` | `ChildrenFn` | required |
| `placement` | `PopoverPlacement` | `PopoverPlacement::default()` |
| `motion` | `PopoverMotion` | `PopoverMotion::default()` |
| `is_modal` | `bool` | `true` |
| `class_name` | `Option<String>` | `None` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |

## Hello World（最小可用）

```rust
let anchor_ref = NodeRef::<html::Button>::new();
let (open, set_open) = signal(false);

<button node_ref=anchor_ref on:click=move |_| set_open.set(true)>"Open"</button>
<Popover
  open=Signal::derive(move || open.get())
  anchor_ref=anchor_ref
  on_close=OnPress::new(move |_| set_open.set(false))
>
  {move || view! { <div>"Popover content"</div> }}
</Popover>
```

## Semantics and Accessibility

- 根节点标记：`data-state`、`data-modal`、`data-placement`。
- 来源标记：`data-motion-source`、`data-placement-source`、`data-modal-source`、`data-class-source`、`data-exit-source`。
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

- `crates/ui-components/src/popover/mod.rs`
- `crates/ui-components/src/popover/logic.rs`
- `crates/ui-components/src/popover/view.rs`
- `crates/ui-components/src/popover/motion.rs`
- `crates/ui-components/src/popover/styles.rs`
