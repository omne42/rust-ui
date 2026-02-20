# LinkButton

`LinkButton` 把按钮视觉与链接语义组合在一起，覆盖外链安全属性与禁用退化行为。

## 展示（Display）

docs-app `link-button` 页面包含多场景对比：

- `External target + rel hardening`（交互配置）
- `Variant + size + disabled matrix`

可同时比较新窗口打开、rel 处理、尺寸矩阵与禁用态。

## config

关键配置轴：

- `variant`
- `size`
- `disabled`
- `target`（是否 `_blank`）
- `rel`（例如 sponsored）
- `href`（含空值归一行为）

## code

```rust
<LinkButton
  href="https://example.com/docs".to_string()
  target="_blank"
  rel="sponsored".to_string()
  variant=ButtonVariant::Outline
>
  "Open docs"
</LinkButton>
```

## css test

- 样式来源：`crates/ui-components/src/button/link_button/styles.rs`
- docs Playground 用状态矩阵验证链接按钮在不同语义态下的样式稳定性。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/actions.rs` (`link_button()`)
