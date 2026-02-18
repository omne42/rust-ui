# Toaster

`Toaster` 是通知宿主组件，负责把 toast 队列稳定挂载到页面中，并输出可测试的语义标记。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
<Toaster />
```

- 默认路径不需要用户手动接线 `ui-state-primitives` / `ui-headless`。
- 先挂载组件即可，后续再按需增加高级参数。

## 常见用法

```rust
let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster store=store.clone() />
store.push_simple("Synced");
```

```rust
<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopCenter
  max_toasts=2
/>
```

## 再进阶（高级控制）

```rust
let custom_motion = ToastMotion {
  initial_y_px: 20.0,
  initial_scale: 0.95,
  ..ToastMotion::default()
};

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopLeft
  max_toasts=4
  aria_label="Alert stream".to_string()
  class_name="docs-toaster-source".to_string()
  motion=custom_motion
/>
```

- 默认 API 路径优先，只有在需要时再开启 `aria_label` / `class_name` / `motion` 等进阶参数。
- `store` 解析顺序固定为 `provided -> context -> local`，可通过 `data-store-source` 观察。

## docs-app 等价入口

- `apps/docs-app/src/pages/components/pages/overlays_extra.rs` 的 `toaster()` 页面。
- Playground 顺序保持“默认在前，进阶在后”：`Hello World -> Portal Queue Host -> Inline Top-Center Host -> State + Source Markers`。
