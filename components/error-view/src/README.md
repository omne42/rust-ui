# ErrorView

`ErrorView` 是一个字段级/局部错误展示组件。默认用法只需要两件事：`is_invalid` + `message`。

## Hello World

```rust
<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
/>
```

## 常见用法

1. 基础显隐（常见）
   - `is_invalid=true` 时显示，`is_invalid=false` 时隐藏（`data-state` 会稳定输出 `visible/hidden`）。
2. 视觉语气和密度
   - 使用 `tone=ErrorViewTone::Neutral`、`is_compact=true`、`is_bordered=true` 调整视觉表现。
3. 自定义内容与操作
   - 通过 `children`、`icon`、`actions` 插槽挂载额外内容（例如重试按钮）。

## 新手路径（先用起来，再进阶）

1. 先用默认路径：只传 `is_invalid` 和 `message`。
2. 再加常见样式参数：`tone`、`is_compact`、`is_bordered`。
3. 最后再启用高级能力：`icon`、`actions`、`motion`、`class_name`、`lang`、`dir`。

## API 约定

- 布尔输入使用 `is_*`：`is_invalid`、`is_compact`、`is_bordered`。
- `ErrorView` 无受控/非受控状态轴，不要求手动接线状态机对象。
- docs-app 入口：`apps/docs-app/src/pages/components/pages/display_extra.rs::error_view()`。

## Source-first

- 组件源码：`components/error-view/src/{mod,logic,view,styles,motion}.rs`
- package feature：`component-error_view`（可选叠加 `inject-css`）
