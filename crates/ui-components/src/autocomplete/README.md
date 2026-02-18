# Autocomplete

`Autocomplete` 是组合输入组件，默认消费完整快照配置（snapshot）并稳定渲染；状态不变量来自 `ui-state-primitives`，键盘与 A11y 语义通过 `ui-headless` 契约挂载。

## Streaming 策略

- `Snapshot`：默认路径，组件稳定消费完整配置并渲染。
- `Streaming Optional`：`Autocomplete` 不是 LLM 正文阅读面；若上层为流式容器，本组件按 `fallback=snapshot` 方式消费稳定配置。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::Autocomplete;

let (selected, set_selected) = signal(None::<usize>);

view! {
    <Autocomplete
        id_base="city".to_string()
        label="City".to_string()
        items=vec!["Tokyo".to_string(), "Osaka".to_string()]
        selected_index=selected
        set_selected_index=set_selected
    />
}
```

## 受控 open 示例

```rust
use leptos::prelude::*;
use ui_components::Autocomplete;

let (selected, set_selected) = signal(Some(0_usize));
let (open_raw, set_open_raw) = signal(false);
let open = Signal::derive(move || open_raw.get());
let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

view! {
    <Autocomplete
        id_base="city-controlled".to_string()
        label="City".to_string()
        items=vec!["Tokyo".to_string(), "Osaka".to_string(), "Nagoya".to_string()]
        selected_index=selected
        set_selected_index=set_selected
        is_open=open
        on_open_change=on_open_change
    />
}
```

## API 约定

- open 受控/非受控轴：`is_open` + `on_open_change` + `default_open`
- 布尔状态：`is_disabled` / `is_required` / `is_invalid`
- 兼容别名：`open`、`disabled`、`required`、`invalid`
- A11y / i18n：支持 `lang` / `dir`，空态文案来源为 `empty_message > i18n > default`
- 语义观测：根节点输出 `data-state`、`data-open`、`data-controlled`、`data-*-source` 等稳定标记

## Source-first

- 组件源码：`crates/ui-components/src/autocomplete/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/autocomplete.rs`
