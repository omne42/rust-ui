# Checkbox / CheckboxGroup

`Checkbox` 提供单项勾选交互，`CheckboxGroup` 提供字段级分组语义（label/description/error/required/invalid）。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可观测（`data-*` / `aria-*`）、可组合的勾选能力。
- 非目标：不负责业务级表单校验编排与全局状态管理。
- 风险边界：状态归一与来源标记集中在 `logic.rs`，`view.rs` 仅挂载结构与语义。

## Architecture Layers

- `logic.rs`：`Checkbox` 与 `CheckboxGroup` 状态归一、label/text 归一、A11y attrs 派生。
- `motion.rs`：`CheckboxMotion` 与 `CheckboxGroupMotion` 契约清洗。
- `view.rs`：Leptos 结构与 headless hooks 挂载。
- `styles.rs`：静态 CSS（`CSS` + `CHECKBOX_GROUP_CSS`）。
- `mod.rs`：对外导出稳定 API（`Checkbox`、`CheckboxGroup` 及类型）。

## API (Table)

### Checkbox Props

| Prop | Type | Default |
| --- | --- | --- |
| `checked` | `ReadSignal<bool>` | required |
| `set_checked` | `WriteSignal<bool>` | required |
| `disabled` | `bool` | `false` |
| `on_change` | `Option<Callback<bool>>` | `None` |
| `variant` | `CheckboxVariant` (`Default` / `Accent`) | `Default` |
| `size` | `CheckboxSize` (`Default` / `Sm` / `Lg`) | `Default` |
| `motion` | `CheckboxMotion` | `CheckboxMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None` |

### CheckboxGroup Props

| Prop | Type | Default |
| --- | --- | --- |
| `id` | `String` | required |
| `label` | `String` | required（空串会归一到 `Options`） |
| `description` | `Option<String>` | `None` |
| `error` | `Option<String>` | `None` |
| `invalid` | `Signal<bool>` | `false` |
| `required` | `Signal<bool>` | `false` |
| `disabled` | `bool` | `false` |
| `aria_describedby` | `Signal<Option<String>>` | `None` |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
let (checked, set_checked) = signal(false);

view! {
    <Checkbox checked=checked set_checked=set_checked>"Accept terms"</Checkbox>
}
```

## Semantics and Accessibility

- `Checkbox` 暴露 `data-state/data-checked/data-unchecked/data-disabled/data-focus-visible`。
- `CheckboxGroup` 暴露 `data-invalid/data-required/data-has-description/data-shows-error`。
- `CheckboxGroup` 使用 `fieldset/legend` 并通过 `use_text_field` 统一 `aria-describedby/aria-invalid/aria-required`。

## Motion and Styling

- `Checkbox` 使用 spring 动效（root + indicator）。
- `CheckboxGroupMotion` 提供分组时长变量契约（用于主题或容器过渡扩展）。
- 样式文件分离：`CSS`（checkbox）与 `CHECKBOX_GROUP_CSS`（group）。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：实时展示 Checkbox / CheckboxGroup 的渲染与语义标记。
- `Config`：切换 checked/disabled/invalid/required/variant/size 等配置。
- `Code`：输出当前配置对应的调用代码。
- `CSS Test`：注入组件样式源码用于 scoped 验证。
- `对比`：至少保留多组状态矩阵（受控、禁用、校验失败、可选分组）进行横向对比。
