# Card

`Card` 是一个轻量的容器组件，用于承载内容块并暴露稳定的样式语义标记。

## 目标 / 非目标 / 风险边界

- 目标：提供统一的 `variant/padded` 视觉语义与可测试 `data-*` 状态标记。
- 非目标：不在组件层实现交互状态机、异步流程或 overlay/focus 行为。
- 风险边界：状态归一化应始终留在 `logic.rs`，避免在 `view.rs` 追加隐式兜底。

## Architecture Layers

- `logic.rs`：`CardVariant` 枚举、输入归一化、状态派生、class 组装。
- `view.rs`：Leptos 结构渲染与 `data-*` 标记挂载。
- `styles.rs`：token-first 静态 CSS（`var(--ui-*)`）。
- `mod.rs`：公开最小稳定 API（`Card`、`CardVariant`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `variant` | `CardVariant` (`Default` / `Muted` / `Outline`) | `Default` |
| `padded` | `bool` | `true` |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
<Card>
  <div>"Card content"</div>
</Card>
```

## Semantics and Accessibility

- 根节点输出稳定标记：`data-slot="card"`、`data-variant`、`data-state`。
- 同步输出来源/分支标记：`data-padded`、`data-flush`、`data-custom-class`。
- `Card` 为语义容器，不引入额外键盘/焦点协议。

## Motion and Fallback

- N/A：`Card` 不包含组件级动效契约与运行时动画驱动。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/layout.rs`
- 页面：`card()`
- Playground：`Variants`、`Padding States`、`Custom Class`、`Workbench (Display + Config + Code + CSS Test)`

## Playground 展示区（Display / Config / Code / CSS Test）

- 展示（Display）：预览 `variant + padded + class source` 的实时组合效果。
- 配置（Config）：通过 Workbench 控件切换 `variant/padded/custom class`，并输出 `CardActualConfig`。
- 代码（Code）：按当前配置生成可复制代码片段，和预览保持一致。
- CSS Test：加载 `card/styles.rs` 原始 CSS，支持 scoped live-edit 与快速回滚。

## 多场景对比展示

- `Variants`：`Default / Muted / Outline` 并排对比视觉层级与状态标记。
- `Padding States`：`padded=true/false` 对比内容密度与 `data-state`。
- `Custom Class`：默认样式与 `docs-card-custom` 合并路径对比。
- `Workbench`：在同一画布里组合对比 `variant + padded + custom class` 多状态。

## Source-first

- `crates/ui-components/src/card/mod.rs`
- `crates/ui-components/src/card/logic.rs`
- `crates/ui-components/src/card/view.rs`
- `crates/ui-components/src/card/styles.rs`
