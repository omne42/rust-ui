# CommandDialog

`CommandDialog` 组合了 `Modal + Command`，提供命令检索面板；默认支持 `Snapshot` 渲染，并持续输出稳定 `data-*` 语义标记。

## Streaming 策略

- `Snapshot`：默认路径，消费完整配置并稳定渲染。
- `Streaming Optional`：组件不是 LLM 正文阅读面；若上层处于流式生成，组件走 `fallback=snapshot`。

## Hello World

```rust
use std::sync::Arc;
use ui_components::{CommandDialog, CommandGroup, CommandItem};

let groups: Arc<[CommandGroup]> = Arc::from(vec![
    CommandGroup::new("Navigation", vec![CommandItem::new("go-home", "Go Home")]),
]);

view! { <CommandDialog groups=groups /> }
```

## 受控打开状态

```rust
use leptos::prelude::*;
use std::sync::Arc;
use ui_components::{CommandDialog, CommandGroup, CommandItem};

let groups: Arc<[CommandGroup]> = Arc::from(vec![
    CommandGroup::new("Actions", vec![CommandItem::new("run-tests", "Run Tests")]),
]);
let (open_raw, set_open_raw) = signal(false);
let open = Signal::derive(move || open_raw.get());
let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

view! {
    <CommandDialog
        groups=groups
        open=open
        on_open_change=on_open_change
    />
}
```

## API 约定

- 打开状态轴：`open + on_open_change + default_open`
- 行为轴：`close_on_action`（默认 `true`）
- 可观测契约：`data-state`、`data-open-mode`、`data-*-source`、`data-ui-*`、`data-stream-*`

## Source-first

- 组件源码：`crates/ui-components/src/command_dialog/{mod,logic,motion,styles,view}.rs`
- 状态/交互原语：`ui_headless::use_controllable_open_state_traced`、`ui_headless::use_presence`
