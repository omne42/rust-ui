# PickerButton

`PickerButton` 是 `FieldButton` 命名对齐包装，保留同一状态语义与可访问契约。

## 展示（Display）

docs-app `picker-button` 页面包含多种对比：

- `Interactive`（带 config 面板）
- `State Matrix`
- `State + Source Markers`

用于并排验证 quiet/invalid/active/disabled 与 marker 输出。

## config

Workbench 控制项：

- `preset`（default / quiet / invalid）
- `disabled`
- `force active`
- `custom aria label`

## code

```rust
<PickerButton
  quiet=true
  invalid=true
  is_active=true
  aria_label="Inspect picker trigger".to_string()
>
  "Choose item"
</PickerButton>
```

## css test

- 样式来源：`crates/ui-components/src/button/picker_button/styles.rs`
- docs Playground 支持 CSS test，结合 `data-state` 与 source markers 做回归验证。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/actions_extra_picker_button.rs` (`picker_button()`)
