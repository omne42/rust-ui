# CloseButton

`CloseButton` 提供标准关闭动作入口，内建默认图标回退与尺寸/状态契约。

## 展示（Display）

docs-app `close-button` 页面提供多场景对比：

- `Default + OverBackground + Custom Label`
- `Size Matrix + Disabled + Custom Class`

通过同屏展示对比尺寸、禁用态与来源标记。

## config

重点配置轴：

- `variant`（default / over-background）
- `size`（sm / md / lg / xl）
- `disabled`
- `aria_label`
- `class_name`

## code

```rust
<CloseButton />
<CloseButton variant=CloseButtonVariant::OverBackground />
<CloseButton size=CloseButtonSize::Lg disabled=true />
```

## css test

- 样式来源：`crates/ui-components/src/button/close_button/styles.rs`
- 在 docs Playground 中可做 CSS test，验证 size/state/source 标记不漂移。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/actions_extra.rs` (`close_button()`)
