# EmptyState

`EmptyState` 是一个基础展示组件，用于在“暂无内容/暂无结果”场景下输出统一语义与视觉状态。

## 文档入口

- docs-app: `/#/components/empty-state`
- 页面源码：`apps/docs-app/src/pages/components/pages/display_extra.rs` 中 `empty_state()`

## 先用起来（Hello World）

不需要先理解分层架构，先用默认 API：

```rust
use leptos::prelude::*;
use ui::EmptyState;

view! { <EmptyState /> }
```

## 常见用法（默认路径优先）

### 1) 基础文案

```rust
use leptos::prelude::*;
use ui::EmptyState;

view! {
    <EmptyState
        title="Nothing matched".to_string()
        description="Try a different query or clear filters.".to_string()
    />
}
```

### 2) 语义状态参数（tone / align / compact / bordered）

```rust
use leptos::prelude::*;
use ui::{EmptyState, EmptyStateAlign, EmptyStateTone};

view! {
    <EmptyState
        title="Deployments paused".to_string()
        description="Approvals are required before resuming this environment.".to_string()
        tone=EmptyStateTone::Accent
        align=EmptyStateAlign::Center
        is_compact=true
        is_bordered=true
        class_name="docs-empty-state-custom".to_string()
    />
}
```

## 进阶（需要时再看）

### Architecture Layers

- `ui-state-primitives::empty_state`：默认值与状态归一（`DEFAULT_*`、`resolve_state`）
- `components/empty-state/src/logic.rs`：props 归一与渲染态派生（`resolve_defaults`、`resolve_render_state`）
- `components/empty-state/src/view.rs`：语义挂载（`data-*` / `aria-*`）与结构渲染
- `components/empty-state/src/motion.rs`：动效 contract 与 attach（含 non-wasm 降级）

### API (Quick Reference)

- `title: Option<String>`
- `description: Option<String>`
- `tone: EmptyStateTone`
- `align: EmptyStateAlign`
- `is_compact: bool`
- `is_bordered: bool`
- `aria_label: Option<String>`
- `class_name: Option<String>`
- `icon: Option<ViewFn>`
- `actions: Option<ViewFn>`
