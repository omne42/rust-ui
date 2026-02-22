# CircularProgress

`CircularProgress` 是一个轻量的环形加载指示器，默认用于展示不确定进度（indeterminate）。

## Hello World（先用起来）

```rust
use leptos::prelude::*;
use ui::CircularProgress;

view! {
    <CircularProgress />
}
```

## 常见用法

### 1) 调整尺寸与粗细

```rust
use leptos::prelude::*;
use ui::CircularProgress;

view! {
    <CircularProgress
        aria_label="Syncing mail".to_string()
        size_px=24.0
        thickness_px=3.0
    />
}
```

### 2) 覆盖语义标签与样式类

```rust
use leptos::prelude::*;
use ui::CircularProgress;

view! {
    <CircularProgress
        aria_label="Background refresh".to_string()
        class_name="docs-circular-progress-custom".to_string()
    />
}
```

## 进阶用法（需要时再看）

### 1) 显式挂载语言与方向上下文

```rust
use leptos::prelude::*;
use ui::color::area::A11yDirection;
use ui::CircularProgress;

view! {
    <CircularProgress
        lang="ar".to_string()
        dir=A11yDirection::Rtl
    />
}
```

### 2) 与上层状态映射（组件自身无受控/非受控状态轴）

```rust
use leptos::prelude::*;
use ui::CircularProgress;

let upstream_label = "Syncing mail".to_string();

view! {
    <CircularProgress aria_label=upstream_label size_px=24.0 />
}
```

## docs-app 入口

- 组件页面：`apps/docs-app/src/pages/components/pages/display.rs` 的 `circular_progress()`
- 页面路由：`/#/components/circular-progress`
