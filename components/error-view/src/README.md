# ErrorView

`ErrorView` 用于展示字段级/局部错误信息，使用 canonical API：`is_compact`、`is_bordered`。

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/display_extra.rs::error_view()`

对比场景：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Hidden | `is_invalid=false` | `data-state="hidden"` |
| Invalid | `is_invalid=true` | `data-state="visible"` |
| Compact | `is_compact=true` | `data-compact="true"` |
| Bordered | `is_bordered=true` | `data-bordered="true"` |

## config（Actual Config）

```text
ErrorViewActualConfig {
  tone: Neutral,
  is_invalid: true,
  is_compact: true,
  is_bordered: true,
  has_icon: true,
  has_actions: true,
}
```

## code（Copy/Paste）

```rust
<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  is_compact=true
  is_bordered=true
  message="Validation failed".to_string()
/>
```

## css test（Scoped CSS Test）

- `test_source_path="components/error-view/src/styles.rs"`
- `test_css_source=ui_components::error_view::styles::CSS`

```css
:scope .ui-error-view[data-state="visible"] {
  outline: 1px dashed var(--ui-danger);
}
```
