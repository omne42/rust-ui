# AvatarGroup

`AvatarGroup` renders a roster of avatars with built-in empty/stable/overflow semantics.

## 先用起来（Quick Start / Hello World）

先走默认 API 路径，不需要先理解分层架构，也不需要手动接线状态机。

```rust
use leptos::prelude::*;
use ui_components::{AvatarGroup, AvatarGroupItem};

view! {
    <AvatarGroup items=Vec::<AvatarGroupItem>::new() />
}
```

## 常见用法（Common Usage）

### 1) 基础头像组 + overflow

```rust
use leptos::prelude::*;
use ui_components::{AvatarGroup, AvatarGroupItem, AvatarSize};

view! {
    <AvatarGroup
        items=vec![
            AvatarGroupItem { name: Some("Ada Lovelace".to_string()), src: None, alt: Some("Ada".to_string()) },
            AvatarGroupItem { name: Some("Grace Hopper".to_string()), src: None, alt: Some("Grace".to_string()) },
            AvatarGroupItem { name: Some("Alan Turing".to_string()), src: None, alt: Some("Alan".to_string()) },
        ]
        max=2
        size=AvatarSize::Md
    />
}
```

### 2) 自定义 aria 与 class

```rust
use leptos::prelude::*;
use ui_components::{AvatarGroup, AvatarGroupItem, AvatarSize};

view! {
    <AvatarGroup
        items=Vec::<AvatarGroupItem>::new()
        size=AvatarSize::Md
        aria_label="No collaborators".to_string()
        class_name="docs-avatar-group-custom".to_string()
    />
}
```

## 默认参数（Defaults）

- `items: Vec<AvatarGroupItem>`: 必填输入。
- `max: Option<usize>`: 默认 `None`，归一化为 `4`。
- `size: AvatarSize`: 默认 `AvatarSize::Md`。
- `aria_label / class_name / lang / dir`: 默认 `None`，按逻辑层归一化和 i18n 上下文处理。

## 进阶（Advanced，按需使用）

- `lang`/`dir`: 显式覆盖语言和方向语义。
- 通过 `data-state`、`data-aria-label-source`、`data-class-source` 观察状态来源契约。
- `AvatarGroup` 没有受控/非受控状态机轴；上层只需传完整快照 props。

先用上面的 Quick Start 和 Common Usage，再按需进入这些进阶能力。

## docs-app 入口

- 页面函数：`apps/docs-app/src/pages/components/pages/display.rs` 的 `avatar_group()`
- 路由：`/#/components/avatar-group`
- Playground：`Hello World`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Source-first Starter`

## Layering

- `logic.rs`: normalization and group render-state derivation.
- `view.rs`: Leptos render + headless group a11y wiring.
- `styles.rs`: token-first static CSS.
- `mod.rs`: minimal public exports.
