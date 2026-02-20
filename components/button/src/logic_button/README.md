# LogicButton

`LogicButton` 提供布尔逻辑动作按钮（AND/OR）展示与交互承载。

## 展示（Display）

docs-app `logic-button` 页面对比展示：

- `AND + OR variants`
- `Custom class + Disabled`

用于验证逻辑模式、禁用态、自定义类来源在同屏下的一致行为。

## config

主要配置轴：

- `variant`（and / or）
- `disabled`
- `class_name`
- `on_press`

## code

```rust
<LogicButton variant=LogicButtonVariant::And>
  "AND"
</LogicButton>
<LogicButton variant=LogicButtonVariant::Or disabled=true>
  "Disabled"
</LogicButton>
```

## css test

- 样式来源：`crates/ui-components/src/button/logic_button/styles.rs`
- docs Playground 可进行 CSS test，对比 variant 与 disabled 组合下的状态样式。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/actions_extra.rs` (`logic_button()`)
