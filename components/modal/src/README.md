# Modal

`Modal` 是一个基于 `Overlay` 组装的对话层组件，重点提供标题/描述/样式来源的稳定状态标记与可追踪语义契约。

## 目标 / 非目标 / 风险边界

- 目标：在 overlay 容器内提供清晰的 `title/description/body` 槽位和可测状态来源标记。
- 非目标：不在组件层实现完整对话框状态机（open 生命周期由外层控制）。
- 风险边界：`id/title/description/class/motion/exit` 的来源判断必须集中在 `logic.rs`。

## Architecture Layers

- `logic.rs`：文本归一化、slot 状态派生、source marker 统一生成。
- `motion.rs`：复用 overlay motion contract 并做输入归一化。
- `view.rs`：渲染 `Overlay` 与 modal slot，并挂载 `aria-*` + `data-*` 契约。
- `styles.rs`：静态样式与状态选择器。
- `mod.rs`：导出 `Modal` 及 slot/state 输入输出结构。

## API (Table)

### Modal Props

| Prop | Type | Default |
| --- | --- | --- |
| `is_open` | `Option<Signal<bool>>` | `None`（空值按 `false`） |
| `default_open` | `Option<bool>` | `None`（`false`） |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `id_base` | `String` | required（空值回退 `ui-modal`） |
| `title` | `String` | required（空值回退 `Modal`） |
| `on_close` | `OnPress` | required |
| `description` | `Option<String>` | `None` |
| `motion` | `OverlayMotion` | `OverlayMotion::default()` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |
| `lang` | `Option<String>` | `None`（可透传 locale 上下文） |
| `dir` | `Option<A11yDirection>` | `None`（LTR/RTL） |
| `class_name` | `Option<String>` | `None` |
| `children` | `ChildrenFn` | required |

### Modal Events

| Event | Type | Default |
| --- | --- | --- |
| `on_open_change` | `Callback<bool>` | `None` |
| `on_close` | `OnPress` | required |
| `on_exit_complete` | `Callback<()>` | `None` |

## Hello World（最小可用）

> 默认路径：无需手动接线 `ui-state-primitives` / `ui-headless` 状态机；组件内部处理 open 轴。

```rust
<Modal default_open=true id_base="m".to_string() title="Hello".to_string() on_close=Callback::new(|_| {})>
  <div>"Minimal modal content"</div>
</Modal>
```

## 先用起来，再进阶

- 默认路径：先用 `default_open + id_base + title + on_close`，不需要先理解底层状态分层。
- 进阶控制：按需启用 `is_open + default_open + on_open_change`（受控/非受控成对）。

## 常见用法

### Controlled Example（高级入口）

```rust
let (open, set_open) = signal(false);
let close: OnPress = Callback::new(move |_| set_open.set(false));

<Modal
  is_open=Signal::derive(move || open.get()).into()
  on_open_change=Callback::new(move |next| set_open.set(next))
  id_base="docs-modal-controlled".to_string()
  title="Confirm".to_string()
  on_close=close
>
  ...
</Modal>
```

## Composite API Boundary（N/A）

- `Modal` 不是集合型组件，不提供 `Item` 列表注册语义。
- 不提供 `labels + children`、`titles + panels` 这类并行数组/并行槽位 API。

## Slots and State Markers

- `ModalSlot`：
  - `Root` -> `data-slot="modal"`
  - `Title` -> `data-slot="modal-title"`
  - `Description` -> `data-slot="modal-description"`
  - `Body` -> `data-slot="modal-body"`
- 根节点状态：
  - `data-state`：`with-description` / `title-only`
  - `data-description`：`present` / `absent`
- 来源标记：
  - `data-id-source`
  - `data-title-source`
  - `data-description-source`
  - `data-class-source`
  - `data-motion-source`
  - `data-exit-source`

## Semantics and Accessibility

- 始终输出 `aria-labelledby` 绑定标题。
- 仅在 `description` 存在时输出 `aria-describedby`，避免空语义引用。
- `aria-labelledby`/`aria-describedby` + locale 归一优先复用 `ui_headless::overlay_dialog_attrs`，避免组件层重复发明 A11y 工具。
- 支持 `lang`/`dir` 透传（`A11yDirection::{Ltr,Rtl}`），不假设单语言/单方向。
- 所有关键状态与来源都可通过稳定 `data-*` 标记检索，适配语义测试与自动化选择器。

## Motion and Fallback

- 动效契约复用 `OverlayMotion`，组件层只负责接线，不重写底层动效执行。
- 未提供 `on_exit_complete` 时使用 no-op 回调，保证调用路径稳定。
- open 轴支持受控/非受控成对：`is_open + on_open_change + default_open`。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：预览 modal 打开关闭、title/description/body 槽位和状态标记。
- `Config`：可切换 id/title/description/class/motion/exit 的来源组合（default/custom）。
- `Code`：实时输出对应 `Modal` 调用代码，便于复制验证。
- `CSS Test`：加载 `modal/styles.rs` 原始样式并支持 scoped 覆盖测试。
- `对比`：页面保留 `Label + Description` 与 `State + Source Markers` 对比场景。
