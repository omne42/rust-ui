# Modal

`Modal` 是一个基于 `Overlay` 组装的对话层组件，重点提供标题/描述/样式来源的稳定状态标记与可追踪语义契约。

## 目标 / 非目标 / 风险边界

- 目标：在 overlay 容器内提供清晰的 `title/description/body` 槽位和可测状态来源标记。
- 非目标：不在组件层实现完整对话框状态机（open 生命周期由外层控制）。
- 风险边界：`id/title/description/class/motion/exit` 的来源判断必须集中在 `logic.rs`。

## Architecture Layers

- `logic.rs`：文本归一化、slot 状态派生、source marker 统一生成。
- `view.rs`：渲染 `Overlay` 与 modal slot，并挂载 `aria-*` + `data-*` 契约。
- `styles.rs`：静态样式与状态选择器。
- `mod.rs`：导出 `Modal` 及 slot/state 输入输出结构。

## API (Table)

### Modal Props

| Prop | Type | Default |
| --- | --- | --- |
| `open` | `Signal<bool>` | required |
| `id_base` | `String` | required（空值回退 `ui-modal`） |
| `title` | `String` | required（空值回退 `Modal`） |
| `on_close` | `OnPress` | required |
| `description` | `Option<String>` | `None` |
| `motion` | `OverlayMotion` | `OverlayMotion::default()` |
| `on_exit_complete` | `Option<Callback<()>>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `children` | `ChildrenFn` | required |

### Modal Events

| Event | Type | Default |
| --- | --- | --- |
| `on_close` | `OnPress` | required |
| `on_exit_complete` | `Callback<()>` | `None` |

## Hello World（最小可用）

```rust
let (open, set_open) = signal(true);
let on_close = Callback::new(move |_| set_open.set(false));

view! {
    <Modal
        open=open.into()
        id_base="docs-modal".to_string()
        title="Confirm".to_string()
        on_close=on_close
    >
        <button on:click=move |_| set_open.set(false)>"Close"</button>
    </Modal>
}
```

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
- 所有关键状态与来源都可通过稳定 `data-*` 标记检索，适配语义测试与自动化选择器。

## Motion and Fallback

- 动效契约复用 `OverlayMotion`，组件层只负责接线，不重写底层动效执行。
- 未提供 `on_exit_complete` 时使用 no-op 回调，保证调用路径稳定。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：预览 modal 打开关闭、title/description/body 槽位和状态标记。
- `Config`：可切换 id/title/description/class/motion/exit 的来源组合（default/custom）。
- `Code`：实时输出对应 `Modal` 调用代码，便于复制验证。
- `CSS Test`：加载 `modal/styles.rs` 原始样式并支持 scoped 覆盖测试。
- `对比`：页面保留 `Label + Description` 与 `State + Source Markers` 对比场景。
