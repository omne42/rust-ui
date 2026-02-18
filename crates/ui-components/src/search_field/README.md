# SearchField

`SearchField` 是搜索输入组件，默认支持 snapshot 渲染；状态派生来自 `ui-state-primitives`，交互语义通过 `ui-headless` 挂载。

## Streaming 策略

- `Snapshot`：默认路径，组件稳定消费完整配置并渲染。
- `Streaming Optional`：`SearchField` 不是 LLM 正文阅读面；若上层是流式输出容器，本组件按 `fallback=snapshot` 消费稳定配置。

## Hello World

```rust
use ui_components::SearchField;

view! { <SearchField id="search".to_string() label="Search".to_string() default_value="rust".to_string() /> }
```

## 受控用法

```rust
use leptos::prelude::*;
use ui_components::SearchField;

let (value_raw, set_value_raw) = signal(String::new());
let value = Signal::derive(move || value_raw.get());
let on_value_change = Callback::new(move |next: String| set_value_raw.set(next));

view! {
    <SearchField
        id="search-controlled".to_string()
        label="Search".to_string()
        value=value
        on_value_change=on_value_change
        default_value="prefill".to_string()
    />
}
```

## API 约定

- 受控/非受控值轴：`value` + `on_value_change` + `default_value`
- 布尔状态：`is_disabled` / `is_read_only` / `is_required` / `is_invalid`
- 兼容迁移：保留 `set_value`、`disabled`、`read_only`、`required`、`invalid`
- A11y / i18n：支持 `lang` / `dir`，清空按钮文案来源为 `clear_button_aria_label > i18n > default`
- 语义观测：根节点输出 `data-state` / `data-value` / `data-requirement` / `data-value-*` / `data-clear-label-source`

## Source-first

- 组件源码：`crates/ui-components/src/search_field/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/search_field.rs`
