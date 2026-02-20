# ClearButton

`ClearButton` 是一个轻量清除动作按钮，聚焦于可访问标签、状态来源标记和可预测的交互语义。

## 展示（Display）

docs-app `clear-button` 页面提供对比展示：

- `Default + OverBackground`：默认样式与覆盖背景样式对比。
- `Inset + Focus Mode + Disabled`：内嵌模式、焦点策略、禁用态并排对比。

## config

交互配置可验证以下轴：

- `variant`（default / over-background）
- `inset`（内嵌按钮模式）
- `prevent_focus` / `exclude_from_tab_order`
- `disabled`
- `class_name`（source marker 对比）

## code

```rust
<ClearButton aria_label="Clear query".to_string()>
  "×"
</ClearButton>
<ClearButton
  inset=true
  prevent_focus=true
  aria_label="Clear token".to_string()
>
  "×"
</ClearButton>
```

## css test

- 样式来源：`crates/ui-components/src/button/clear_button/styles.rs`
- docs Playground 支持 scoped CSS test，用于验证 `data-*` 状态标记与样式联动稳定性。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/actions_extra.rs` (`clear_button()`)
