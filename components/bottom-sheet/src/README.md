# BottomSheet

`BottomSheet` 是一个基于 `Sheet` 组合出的底部抽屉组件，聚焦稳定语义标记、可预测关闭行为和最小可用 API 路径。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可观测状态来源的底部浮层容器。
- 非目标：不在组件层内重写状态机，也不承载业务异步协议。
- 风险边界：交互与语义契约漂移时，优先在 `logic.rs` 统一归一化，不在 `view.rs` 打补丁。

## Architecture Layers

- `logic.rs`：props 归一化、默认值、状态派生与来源标记。
- `view.rs`：渲染 `BottomSheet` 结构并挂载 `aria-*` + `data-*`。
- `motion.rs`：`BottomSheetMotion` 契约和 sanitize 映射。
- `styles.rs`：token-first 静态样式规则。
- `mod.rs`：公开最小稳定 API。

## API (Table)

### BottomSheet Props

| Prop | Type | Default |
| --- | --- | --- |
| `open` | `Signal<bool>` | required |
| `on_close` | `OnPress` | required |
| `id_base` | `String` | required（空值回退 `"ui-bottom-sheet"`） |
| `title` | `String` | required（空值回退 `"Bottom sheet"`） |
| `children` | `ChildrenFn` | required |
| `description` | `Option<String>` | `None` |
| `footer` | `Option<ViewFn>` | `None` |
| `motion` | `BottomSheetMotion` | `BottomSheetMotion::default()` |
| `is_handle_visible` | `Option<bool>` | `None`（`true`） |
| `is_close_button_visible` | `Option<bool>` | `None`（`true`） |
| `is_detached` | `Option<bool>` | `None`（`false`） |
| `bottom_inset_px` | `Option<f64>` | `None`（`0.0`） |
| `is_dismissable` | `Option<bool>` | `None`（`true`） |
| `is_keyboard_dismiss_disabled` | `Option<bool>` | `None`（`false`） |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
<BottomSheet
  open=open
  id_base="docs-bottom-sheet".to_string()
  title="Bottom sheet".to_string()
  on_close=on_close
>
  <div>"Hello world"</div>
</BottomSheet>
```

## 先用起来，再进阶

- 默认路径：先用 `open + id_base + title + on_close`
- 进阶控制：按需启用 `description + footer + motion + is_detached`
- 不需要先理解 `ui-state-primitives` / `ui-headless` 内部细节就能直接使用。

## 常见用法

### Common Example（默认路径）

```rust
<BottomSheet
  open=open
  id_base="settings-sheet".to_string()
  title="Settings".to_string()
  description="Configure account and privacy preferences.".to_string()
  on_close=on_close
>
  <div>"Common path content"</div>
</BottomSheet>
```

### Advanced Example（高级入口）

```rust
<BottomSheet
  open=open
  id_base="advanced-sheet".to_string()
  title="Advanced".to_string()
  description="Advanced contract with footer and custom motion.".to_string()
  footer=move || view! { <div>"Footer actions"</div> }
  motion=BottomSheetMotion {
    sheet: ui_components::SheetMotion {
      initial_offset_px: 64.0,
      ..ui_components::SheetMotion::default()
    }
  }
  is_detached=true
  is_close_button_visible=false
  bottom_inset_px=16.0
  on_close=on_close
>
  <div>"Advanced path content"</div>
</BottomSheet>
```

## Semantics and Accessibility

- 标题 id 固定为 `{id_base}-title`，通过 `aria-labelledby` 建立可读名称。
- 仅在 `description` 存在时挂载 `aria-describedby={id_base}-description`。
- 透传 `lang/dir`，支持 LTR/RTL。
- 暴露稳定标记：`data-slot="bottom-sheet"`、`data-state`、`data-open`、`data-ui-*`。

## Source-first Copy-Paste Ready

- 真实源码落点：
  - `components/bottom-sheet/src/mod.rs`
  - `components/bottom-sheet/src/logic.rs`
  - `components/bottom-sheet/src/view.rs`
  - `components/bottom-sheet/src/styles.rs`
  - `components/bottom-sheet/src/motion.rs`
