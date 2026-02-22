# Alert

`Alert` 是一个状态提示组件，用来展示信息、成功、注意和错误提示。

## Hello World（先用起来）

```rust
use leptos::prelude::*;
use ui::Alert;

view! {
    <Alert>
        "Install now to keep your workspace secure."
    </Alert>
}
```

## 常见用法

### 1) 设置语义色和填充样式

```rust
use leptos::prelude::*;
use ui::{Alert, AlertFill, AlertTone};

view! {
    <Alert
        tone=AlertTone::Info
        fill=AlertFill::Border
        title="Updates available".to_string()
        description="A new version is ready to install.".to_string()
    >
        "Install now to keep your workspace secure."
    </Alert>
}
```

### 2) Inline 紧凑布局

```rust
use leptos::prelude::*;
use ui::{Alert, AlertFill, AlertLayout, AlertTone};

view! {
    <Alert
        layout=AlertLayout::Inline
        tone=AlertTone::Notice
        fill=AlertFill::Subtle
        title="Inline mode".to_string()
    >
        "Compact inline content."
    </Alert>
}
```

## 进阶用法（需要时再看）

### 1) 显式覆盖 icon 可见性与读屏标签

```rust
use leptos::prelude::*;
use ui::{Alert, AlertTone};

view! {
    <Alert
        tone=AlertTone::Negative
        is_hide_icon=false
        icon_label="Error notification".to_string()
    >
        "Action required."
    </Alert>
}
```

### 2) 覆盖动效合同参数

```rust
use leptos::prelude::*;
use ui::{Alert, AlertMotion};

view! {
    <Alert
        motion=AlertMotion {
            stiffness: Some(220.0),
            damping: Some(24.0),
            mass: Some(1.0),
        }
    >
        "Custom motion contract."
    </Alert>
}
```

## docs-app 入口

- 组件页面：`apps/docs-app/src/pages/components/pages/display.rs` 的 `alert()`
- 页面路由：`/#/components/alert`

