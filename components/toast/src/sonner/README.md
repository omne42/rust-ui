# Sonner

`Sonner` 是通知宿主组件，负责组合 `ToastViewport` 并输出稳定的状态/来源语义标记。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
<Sonner />
```

- 默认路径不需要用户手动接线 `ui-state-primitives` / `ui-headless`。
- 先挂载组件即可，后续再按需增加高级参数。

## 常见用法

```rust
let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Sonner store=store.clone() />
store.push_simple("Saved");
```

```rust
<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopCenter
  max_toasts=2
/>
```

## 再进阶（高级控制）

```rust
let custom_motion = ToastMotion {
  initial_y_px: 22.0,
  initial_scale: 0.94,
  ..ToastMotion::default()
};

<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopLeft
  max_toasts=4
  aria_label="Status updates".to_string()
  class_name="docs-sonner-source".to_string()
  motion=custom_motion
/>
```

- 默认 API 路径优先，只有在需要时再开启 `aria_label` / `class_name` / `motion` 等进阶参数。
- `store` 解析顺序固定为 `provided -> context -> local`，可通过 `data-store-source` 观察。

## docs-app 等价入口

- `apps/docs-app/src/pages/components/pages/overlays_extra.rs` 的 `sonner()` 页面。
- Playground 顺序保持“默认在前，进阶在后”：`Hello World -> Portal Queue + Variants -> Inline Top-Center + Max Queue -> State + Source Markers`。

## Source-first / Copy-Paste Ready

- docs-app Playground 通过 `Show code` + copy action 输出可直接复制片段，导入拼装统一走 `apps/docs-app/src/playground.rs::compose_copy_ready_code`。
- 组件源码落点：
  - `components/toast/src/sonner/mod.rs`
  - `components/toast/src/sonner/logic.rs`
  - `components/toast/src/sonner/view.rs`
  - `components/toast/src/sonner/styles.rs`
  - `components/toast/src/sonner/motion.rs`
- 依赖前提：`ui-components` 最小 feature 组合为 `component-sonner` 与 `component-toast`。

## 命名兼容策略

- 当前对外命名保持 `portal/max_toasts/aria_label/class_name/motion`，与 `toast/toaster` 同语义同名，不引入同义别名漂移。
- 若后续全库统一推进 `is_*` 命名，采用“先引入兼容别名并标注弃用周期，再移除旧名”的迁移路径，避免破坏现有调用方。
