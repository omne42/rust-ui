# Field

`Field` 是表单字段容器组件，用于统一组织 label/control/description/error，并暴露稳定语义契约。

## 目标 / 非目标 / 风险边界

- 目标：集中管理字段级状态（`required/disabled/invalid`）与文案来源标记（aria/error/class source）。
- 非目标：不承载受控 value 状态机，不直接处理业务异步提交流程。
- 风险边界：不要在 `view.rs` 拼业务状态分支；所有归一化应通过 `logic.rs` 完成。

## Architecture Layers

- `logic.rs`：`FieldOrientation/FieldTone`、文案归一化、状态派生与 source marker。
- `view.rs`：结构渲染（label/control/messages）与 `aria/data-*` 契约挂载。
- `motion.rs`：`FieldMotion` 契约与 CSS 变量 attach（`--ui-field-motion-duration`）。
- `styles.rs`：token-first 静态 CSS。
- `mod.rs`：公开 API（`Field`、`FieldOrientation`、`FieldTone`、`FieldMotion`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `orientation` | `FieldOrientation` (`Vertical` / `Horizontal`) | `Vertical` |
| `tone` | `FieldTone` (`Default` / `Muted`) | `Default` |
| `required` | `bool` | `false` |
| `disabled` | `bool` | `false` |
| `invalid` | `bool` | `false` |
| `label` | `Option<String>` | `None` |
| `description` | `Option<String>` | `None` |
| `error_message` | `Option<String>` | `None`（`invalid=true` 时回落到默认错误文案） |
| `motion` | `FieldMotion` | `FieldMotion::default()` |
| `aria_label` | `Option<String>` | `None`（回落 `DEFAULT_ARIA_LABEL`） |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
<Field label="Email".to_string()>
  <input type="email" />
</Field>
```

## Semantics and Accessibility

- 根节点暴露：`aria-label`、`aria-disabled`、`aria-invalid`。
- 状态标记：`data-state`、`data-message-kind`、`data-required`、`data-disabled`、`data-invalid`。
- 来源标记：`data-aria-source`、`data-error-source`、`data-class-source`。
- 错误消息节点使用 `role="alert"`。

## Motion and Fallback

- motion 仅映射到 CSS 变量（`--ui-field-motion-duration`）。
- `sanitize_motion` 对非法值做回落与 clamp（`1..=800ms`）。
- `@media (prefers-reduced-motion: reduce)` 下样式退化为最小过渡。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms_extra.rs`
- 页面：`field()`
- Playground：`Required + Description`、`Horizontal + Invalid + Custom Class`、`Workbench (Display + Config + Code + CSS Test)`

## Playground 展示区（Display / Config / Code / CSS Test）

- 展示（Display）：实时观察 `orientation/tone/required/invalid/disabled` 组合行为。
- 配置（Config）：Workbench 控件驱动状态与 motion，并输出 `FieldActualConfig`。
- 代码（Code）：根据当前配置生成等价示例代码，便于粘贴复现。
- CSS Test：加载 `field/styles.rs` 样式并支持 scoped CSS 调参测试。

## 多场景对比展示

- `Required + Description`：默认纵向布局下的必填提示与描述信息。
- `Horizontal + Invalid + Custom Class`：横向 + 校验失败 + 自定义类路径对比。
- `Workbench`：在同一画布连续对比 `orientation/tone/invalid/disabled/motion/class` 多状态。

## Source-first

- `crates/ui-components/src/field_form/field/mod.rs`
- `crates/ui-components/src/field_form/field/logic.rs`
- `crates/ui-components/src/field_form/field/view.rs`
- `crates/ui-components/src/field_form/field/motion.rs`
- `crates/ui-components/src/field_form/field/styles.rs`
