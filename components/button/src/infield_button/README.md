# InfieldButton

`InfieldButton` 用于输入框内联动作触发，强调 quiet/invalid/active 的可观测状态契约。

## 展示（Display）

docs-app `infield-button` 页面提供对比：

- `Default + Quiet`
- `Invalid + Active + Disabled`

同屏对比多状态，验证视觉反馈与语义标记一致。

## config

主要配置轴：

- `quiet`
- `invalid`
- `is_active`
- `disabled`
- `aria_label`
- `class_name`

## code

```rust
<InfieldButton quiet=true>
  "Filter"
</InfieldButton>
<InfieldButton invalid=true is_active=true>
  "Apply"
</InfieldButton>
```

## css test

- 样式来源：`crates/ui-components/src/button/infield_button/styles.rs`
- docs Playground 可用于 CSS test，验证 quiet/invalid/active 状态分支。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/actions_extra.rs` (`infield_button()`)
