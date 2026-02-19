# SpinButton

`spinbutton` 在当前结构中仅保留治理清单（`check2.md`），不再作为独立 UI 组件对外导出。

## 展示（Display）

对应展示能力由 `NumberField` 承载，docs-app 提供多场景对比：

- `Stepper`
- `State Matrix`
- `Interactive Playground (Display + Config + Code + CSS Test)`

## config

`NumberField` Playground 控制项覆盖 spinbutton 核心轴：

- bounds（范围）
- step（步进）
- disabled
- required
- invalid

## code

```rust
<NumberField
  id="qty".to_string()
  label="Quantity".to_string()
  value=value
  set_value=set_value
  min=0
  max=100
  step=1
/>
```

## css test

- 样式来源：`crates/ui-components/src/text_input/number_field/styles.rs`
- docs `NumberField` Interactive Playground 提供 CSS test 面板做样式契约验证。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms.rs` (`number_field()`)
- `apps/docs-app/src/pages/components/mod.rs` 中 `"spinbutton" => &["number-field"]`
