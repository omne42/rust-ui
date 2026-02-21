# Drawer

`Drawer` 是一个基于 `Sheet` 组合出的侧边/底部抽屉组件，负责结构语义、状态标记与可预测交互契约。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可受控的抽屉容器（标题、描述、关闭行为、状态/source 标记）。
- 非目标：不在组件层承载业务状态、路由逻辑或全局动画编排。
- 风险边界：交互或可访问性语义漂移时，优先在 `logic.rs` / `sheet` 层修正，不在 `view.rs` 堆叠分支补丁。

## Architecture Layers

- `logic.rs`：参数归一化（`id_base/title/description/class_name`）与状态/source 标记派生。
- `view.rs`：渲染 Drawer 结构并组合 `Sheet`，输出稳定 `data-*` 契约。
- `motion.rs`：`DrawerMotion` 契约（封装 `SheetMotion`）与 sanitize 入口。
- `styles.rs`：静态 CSS 契约，依赖 `data-slot/data-state/data-*-source` 标记。
- `mod.rs`：公开最小稳定 API（`Drawer`、`DrawerPlacement`、`DrawerMotion`）。

## API (Table)

### Drawer Props

| Prop | Type | Default |
| --- | --- | --- |
| `is_open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None` (`false`) |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `on_close` | `Option<OnPress>` | `None` |
| `id_base` | `String` | required（空串会回退为 `"ui-drawer"`） |
| `title` | `String` | required（空串会回退为 `"Drawer"`） |
| `children` | `ChildrenFn` | required |
| `description` | `Option<String>` | `None` |
| `footer` | `Option<ViewFn>` | `None` |
| `placement` | `Option<DrawerPlacement>` (`Right` / `Left` / `Bottom`) | `None` (`Right`) |
| `motion` | `DrawerMotion` | `DrawerMotion::default()` |
| `is_close_button_visible` | `Option<bool>` | `None` (`true`) |
| `close_label` | `Option<&'static str>` | `None` (`"Close"`) |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` (`Ltr` / `Rtl`) | `None` |
| `on_exit_complete` | `Option<Callback<()>>` | `None`（内部归一为 no-op） |
| `class_name` | `Option<String>` | `None` |

### Naming Migration

- `open` -> `is_open`
- `show_close_button` -> `is_close_button_visible`

### Drawer Events

| Event | Type | Default |
| --- | --- | --- |
| `on_open_change` | `Callback<bool>` | optional |
| `on_close` | `OnPress` | optional |
| `on_exit_complete` | `Callback<()>` | optional |

## Hello World（最小可用）

```rust
<Drawer default_open=true id_base="settings-drawer".to_string() title="Settings".to_string()>
  <p>"Drawer content"</p>
</Drawer>
```

## 先用起来，再进阶

- 默认路径：先用 `default_open + id_base + title + on_close`
- 进阶控制：按需启用 `is_open + default_open + on_open_change`
- 不需要先理解 `ui-state-primitives` / `ui-headless` 内部状态机细节即可上手。

## 常见用法

### Uncontrolled Example（默认路径）

```rust
<Drawer
  default_open=true
  id_base="profile-drawer".to_string()
  title="Profile".to_string()
  on_close=Callback::new(|_| {})
>
  <p>"Uncontrolled content"</p>
</Drawer>
```

### Controlled Example（高级入口）

```rust
let (open_raw, set_open_raw) = signal(false);
let open = Signal::derive(move || open_raw.get());

<Drawer
  is_open=open
  default_open=false
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  on_close=Callback::new(move |_| set_open_raw.set(false))
  id_base="advanced-drawer".to_string()
  title="Advanced Drawer".to_string()
>
  <p>"Controlled content"</p>
</Drawer>
```

## Semantics and Accessibility

- 通过 `Sheet` 提供模态容器语义；标题 id 固定为 `{id_base}-title`，绑定 `aria-labelledby`。
- 仅当 `description` 存在时，才渲染描述节点并绑定 `aria-describedby={id_base}-description`。
- `lang` / `dir` 输入透传到 `Sheet`，由 `ui-headless` 的 `overlay_dialog_attrs` 统一归一。
- 暴露稳定插槽标记：`drawer`、`drawer-header`、`drawer-title`、`drawer-description`、`drawer-body`、`drawer-footer`、`drawer-close`。
- 根节点输出状态/source 标记（如 `data-open-state`、`data-open-mode`、`data-open-source`、`data-open-action-source`、`data-placement`、`data-description`、`data-footer`、`data-close-button` 与对应 `data-*-source`），便于样式与测试契约锁定。

## Motion and Fallback

- `DrawerMotion` 仅封装 `sheet: SheetMotion`，默认值与 `SheetMotion::default()` 对齐。
- `sanitize_motion` 委托 `sheet::motion::sanitize_motion`，阻断非法 spring/offset 参数进入运行时。
- 实际进出场动画由 `Sheet` 执行；非 wasm/SSR 场景走 no-op 路径，保证编译与行为可预测。

## Test Contract

- 语义契约测试：`crates/ui-components/tests/drawer_semantics.rs`
- 覆盖范围包含：
  - 模块导出与 crate re-export
  - `logic` 状态/source 派生函数
  - `view` 的 `data-*` 契约与 `aria-describedby` 条件挂载
  - `motion` sanitize 合约
  - CSS 聚合与 docs 入口锚点

## Source-first Copy-Paste Ready

- 真实源码落点：
  - `components/drawer/src/mod.rs`
  - `components/drawer/src/logic.rs`
  - `components/drawer/src/view.rs`
  - `components/drawer/src/styles.rs`
  - `components/drawer/src/motion.rs`
- docs 示例应与上述源码契约同步，避免“复制后接口不匹配”。
