# AlertDialog

`AlertDialog` 是一个 destructive-first 的确认弹层组件，基于 `Overlay` 组合并输出稳定 `role/aria/data-*` 语义契约。

## Hello World（最小可用）

默认路径：先把确认流程跑起来，不需要先理解底层 primitives/headless 分层。

```rust
let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));

<AlertDialog
  open=open
  id_base="docs-alert".to_string()
  title="Delete item?".to_string()
  on_close=on_close
  confirm_label="Delete".to_string()
  on_confirm=Callback::new(move |_| {})
/>
```

## 先用起来，再进阶

- 默认路径：先用 `open + on_close + title + confirm_label`，直接完成基础确认交互。
- 常见增强：按需加 `description` 与 `variant`（`Destructive/Warning/Error`）。
- 进阶控制：再启用 `secondary_label + on_secondary`、禁用态（`is_confirm_disabled/is_secondary_disabled`）、`auto_focus_button`、`motion`。

## 常见用法

- 删除确认：`variant=AlertDialogVariant::Destructive` + `confirm_label="Delete"`。
- 风险提示：`variant=AlertDialogVariant::Warning` 并提供 `secondary_label` 作为保守路径。
- 错误阻断：`variant=AlertDialogVariant::Error` + `is_confirm_disabled=true`，明确不可提交状态。

### Controlled Example（高级入口）

```rust
let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let on_confirm: OnPress = Callback::new(move |_| set_open_raw.set(false));

<AlertDialog
  open=open
  id_base="docs-alert-controlled".to_string()
  title="Controlled alert".to_string()
  description="open: Signal<bool> is the source of truth.".to_string()
  on_close=on_close
  confirm_label="Continue".to_string()
  on_confirm=on_confirm
  secondary_label="Cancel safely".to_string()
  on_secondary=Callback::new(move |_| set_open_raw.set(false))
  variant=AlertDialogVariant::Warning
/>
```

## 文档入口

- docs-app 页面：`/#/components/alert-dialog`
- 组件目录：`components/alert-dialog/src/`
  - `logic.rs`：状态归一化与来源标记
  - `view.rs`：结构渲染 + A11y/语义挂载
  - `motion.rs`：动效 contract 映射
  - `styles.rs`：token-first 静态样式
