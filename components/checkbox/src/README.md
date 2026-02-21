# Checkbox

`Checkbox` 提供单项勾选交互，状态通过 headless 语义与稳定 `data-*` 标记对外可观测。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `is_checked` | `Option<ReadSignal<bool>>` | `None`（优先于 `checked`） |
| `on_checked_change` | `Option<WriteSignal<bool>>` | `None`（优先于 `set_checked`） |
| `default_checked` | `Option<bool>` | `None`（仅在未传受控信号时生效） |
| `is_disabled` | `Option<bool>` | `None`（优先于 `disabled`） |
| `checked`（兼容别名） | `Option<ReadSignal<bool>>` | `None` |
| `set_checked`（兼容别名） | `Option<WriteSignal<bool>>` | `None` |
| `disabled`（兼容别名） | `bool` | `false` |
| `on_change`（兼容别名） | `Option<Callback<bool>>` | `None` |
| `variant` | `CheckboxVariant` | `CheckboxVariant::Default` |
| `size` | `CheckboxSize` | `CheckboxSize::Default` |
| `motion` | `CheckboxMotion` | `CheckboxMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<ui_headless::A11yDirection>` | `None` |

## Hello World（最小可用）

```rust
view! { <Checkbox>"Accept terms"</Checkbox> }
```

## 常见用法

```rust
let (checked, set_checked) = signal(false);

view! {
    <Checkbox is_checked=Some(checked) on_checked_change=Some(set_checked)>
        "Accept terms"
    </Checkbox>
}
```

## 先用起来，再进阶

- 默认路径：先用 `<Checkbox>"Accept terms"</Checkbox>` 完成交互。
- 常见受控：在需要外部状态单一事实来源时使用 `is_checked + on_checked_change`。
- 进阶控制：按需叠加 `default_checked`、`is_disabled`、`variant`、`size`、`motion`。

### Controlled（高级入口）

```rust
let (checked, set_checked) = signal(false);

view! {
    <Checkbox is_checked=Some(checked) on_checked_change=Some(set_checked)>
        "Accept terms"
    </Checkbox>
}
```

## 命名兼容与迁移

- 主命名已切到 `is_checked/on_checked_change/default_checked` 与 `is_disabled`。
- 兼容别名 `checked/set_checked/disabled/on_change` 仍可用，归一化优先级统一在 `logic.rs`。
- 受控但未提供 `on_checked_change` 时，组件将保持只读，不会偷偷写本地状态。
- 迁移建议：先替换 docs 和业务调用到主命名，再逐步删除别名调用。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：实时渲染 checked/unchecked 与禁用状态。
- `Config`：切换 variant/size/motion 与可访问性参数。
- `Code`：输出可复制的最小与进阶用法。
- `CSS Test`：注入样式源码做契约校验。
